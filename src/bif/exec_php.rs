use crate::{bif::BifError, Value};
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::path::Path;

const FCGI_VERSION_1: u8 = 1;
const FCGI_BEGIN_REQUEST: u8 = 1;
const FCGI_END_REQUEST: u8 = 3;
const FCGI_PARAMS: u8 = 4;
const FCGI_STDIN: u8 = 5;
const FCGI_STDOUT: u8 = 6;
const FCGI_STDERR: u8 = 7;
const FCGI_RESPONDER: u16 = 1;
const FCGI_REQUEST_ID: u16 = 1;
const PHP_FPM_TIMEOUT_SECS: u64 = 5;
const BRIDGE_ERROR_KEY: &str = "__neutralts_obj_error";

const BRIDGE_SCRIPT: &str = r#"<?php
header('Content-Type: application/json');
$raw = file_get_contents('php://input');
$payload = json_decode($raw, true);

if (!is_array($payload)) {
    echo json_encode(["__neutralts_obj_error" => "invalid payload"]);
    exit;
}

$script_file = $payload["script_file"] ?? "";
$callback = $payload["callback"] ?? "main";
$params = $payload["params"] ?? [];
$GLOBALS["__NEUTRAL_SCHEMA__"] = $payload["schema"] ?? null;
$GLOBALS["__NEUTRAL_SCHEMA_DATA__"] = $payload["schema_data"] ?? null;

if (!is_string($script_file) || $script_file === "" || !is_file($script_file)) {
    echo json_encode(["__neutralts_obj_error" => "obj script not found"]);
    exit;
}

try {
    require_once $script_file;
} catch (Throwable $e) {
    echo json_encode(["__neutralts_obj_error" => "php script load failed"]);
    exit;
}

if (!is_callable($callback)) {
    echo json_encode(["__neutralts_obj_error" => "callback not found"]);
    exit;
}

try {
    $result = call_user_func($callback, $params);
} catch (Throwable $e) {
    echo json_encode(["__neutralts_obj_error" => "callback execution failed"]);
    exit;
}

$json = json_encode($result);
if ($json === false) {
    echo json_encode(["__neutralts_obj_error" => "invalid callback response"]);
    exit;
}

echo $json;
"#;

enum FpmEndpoint {
    Tcp(SocketAddr),
    #[cfg(unix)]
    Unix(String),
}

enum FpmStream {
    Tcp(TcpStream),
    #[cfg(unix)]
    Unix(UnixStream),
}

impl Read for FpmStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Tcp(s) => s.read(buf),
            #[cfg(unix)]
            Self::Unix(s) => s.read(buf),
        }
    }
}

impl Write for FpmStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Tcp(s) => s.write(buf),
            #[cfg(unix)]
            Self::Unix(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Tcp(s) => s.flush(),
            #[cfg(unix)]
            Self::Unix(s) => s.flush(),
        }
    }
}

pub struct PhpExecutor;

impl PhpExecutor {
    pub(crate) fn exec_php(
        file: &str,
        params_value: &Value,
        callback_name: &str,
        schema: Option<&Value>,
        schema_data: Option<&Value>,
        fpm_endpoint: &str,
    ) -> Result<Value, BifError> {
        let bridge_path = Self::bridge_path_for(file)?;
        Self::ensure_bridge_script(&bridge_path)?;

        let endpoint = Self::parse_endpoint(fpm_endpoint).map_err(|e| BifError {
            msg: e,
            name: "php_callback".to_string(),
            file: file.to_string(),
            src: file.to_string(),
        })?;
        let mut stream = Self::connect_endpoint(&endpoint).map_err(|e| BifError {
            msg: format!("php-fpm connection failed: {}", e),
            name: "php_callback".to_string(),
            file: file.to_string(),
            src: file.to_string(),
        })?;

        let payload = serde_json::json!({
            "script_file": file,
            "callback": callback_name,
            "params": params_value,
            "schema": schema.cloned().unwrap_or(Value::Null),
            "schema_data": schema_data.cloned().unwrap_or(Value::Null),
        });
        let body = serde_json::to_vec(&payload).map_err(|e| BifError {
            msg: format!("invalid php payload: {}", e),
            name: "php_callback".to_string(),
            file: file.to_string(),
            src: file.to_string(),
        })?;

        Self::send_fastcgi_request(&mut stream, &body, &bridge_path).map_err(|e| BifError {
            msg: format!("php-fpm request failed: {}", e),
            name: "php_callback".to_string(),
            file: file.to_string(),
            src: file.to_string(),
        })?;

        let (stdout, stderr) = Self::read_fastcgi_response(&mut stream).map_err(|e| BifError {
            msg: format!("php-fpm response failed: {}", e),
            name: "php_callback".to_string(),
            file: file.to_string(),
            src: file.to_string(),
        })?;

        if !stderr.trim().is_empty() {
            return Err(BifError {
                msg: format!("php-fpm stderr: {}", stderr.trim()),
                name: "php_callback".to_string(),
                file: file.to_string(),
                src: file.to_string(),
            });
        }

        let body = Self::extract_http_body(&stdout);
        let value: Value = serde_json::from_str(body.trim()).map_err(|e| BifError {
            msg: format!("invalid php-fpm response: {}", e),
            name: "php_callback".to_string(),
            file: file.to_string(),
            src: file.to_string(),
        })?;

        if let Some(msg) = value
            .get(BRIDGE_ERROR_KEY)
            .and_then(|v| v.as_str())
            .map(|v| v.to_string())
        {
            return Err(BifError {
                msg,
                name: "php_callback".to_string(),
                file: file.to_string(),
                src: file.to_string(),
            });
        }

        Ok(value)
    }

    fn bridge_path_for(file: &str) -> Result<String, BifError> {
        let file_path = Path::new(file);
        let parent = file_path.parent().unwrap_or_else(|| Path::new("."));
        let parent_abs = if parent.is_absolute() {
            parent.to_path_buf()
        } else {
            std::env::current_dir()
                .map_err(|e| BifError {
                    msg: format!("failed to get current_dir: {}", e),
                    name: "php_callback".to_string(),
                    file: file.to_string(),
                    src: file.to_string(),
                })?
                .join(parent)
        };
        let bridge_path = parent_abs.join(".neutralts_obj_bridge.php");
        let bridge = bridge_path
            .to_str()
            .ok_or_else(|| BifError {
                msg: "invalid bridge path encoding".to_string(),
                name: "php_callback".to_string(),
                file: file.to_string(),
                src: file.to_string(),
            })?
            .to_string();
        Ok(bridge)
    }

    fn ensure_bridge_script(bridge_path: &str) -> Result<(), BifError> {
        if Path::new(bridge_path).exists() {
            return Ok(());
        }

        fs::write(bridge_path, BRIDGE_SCRIPT).map_err(|e| BifError {
            msg: format!("failed to create php bridge: {}", e),
            name: "php_callback".to_string(),
            file: "".to_string(),
            src: "".to_string(),
        })?;
        Ok(())
    }

    fn parse_endpoint(endpoint: &str) -> Result<FpmEndpoint, String> {
        let endpoint = endpoint.trim();
        if endpoint.is_empty() {
            return Err("invalid php-fpm endpoint".to_string());
        }

        if let Some(path) = endpoint.strip_prefix("unix:") {
            if path.is_empty() {
                return Err("invalid php-fpm endpoint".to_string());
            }
            #[cfg(unix)]
            {
                return Ok(FpmEndpoint::Unix(path.to_string()));
            }
            #[cfg(not(unix))]
            {
                return Err("unix socket is not supported on this platform".to_string());
            }
        }

        let tcp_endpoint = endpoint.strip_prefix("tcp://").unwrap_or(endpoint);
        if tcp_endpoint.contains(':') {
            let mut addrs = tcp_endpoint
                .to_socket_addrs()
                .map_err(|_| "invalid php-fpm endpoint".to_string())?;
            if let Some(addr) = addrs.next() {
                return Ok(FpmEndpoint::Tcp(addr));
            }
        }

        Err("invalid php-fpm endpoint".to_string())
    }

    fn connect_endpoint(endpoint: &FpmEndpoint) -> Result<FpmStream, String> {
        match endpoint {
            FpmEndpoint::Tcp(addr) => {
                let stream = TcpStream::connect_timeout(
                    addr,
                    std::time::Duration::from_secs(PHP_FPM_TIMEOUT_SECS),
                )
                .map_err(|e| e.to_string())?;
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(PHP_FPM_TIMEOUT_SECS)))
                    .map_err(|e| e.to_string())?;
                stream
                    .set_write_timeout(Some(std::time::Duration::from_secs(PHP_FPM_TIMEOUT_SECS)))
                    .map_err(|e| e.to_string())?;
                Ok(FpmStream::Tcp(stream))
            }
            #[cfg(unix)]
            FpmEndpoint::Unix(path) => {
                let stream = UnixStream::connect(path).map_err(|e| e.to_string())?;
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(PHP_FPM_TIMEOUT_SECS)))
                    .map_err(|e| e.to_string())?;
                stream
                    .set_write_timeout(Some(std::time::Duration::from_secs(PHP_FPM_TIMEOUT_SECS)))
                    .map_err(|e| e.to_string())?;
                Ok(FpmStream::Unix(stream))
            }
        }
    }

    fn send_fastcgi_request(
        stream: &mut FpmStream,
        body: &[u8],
        bridge_path: &str,
    ) -> Result<(), String> {
        let begin_body = [
            (FCGI_RESPONDER >> 8) as u8,
            (FCGI_RESPONDER & 0xFF) as u8,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        Self::write_record(stream, FCGI_BEGIN_REQUEST, FCGI_REQUEST_ID, &begin_body)?;

        let params = vec![
            ("SCRIPT_FILENAME", bridge_path.to_string()),
            ("SCRIPT_NAME", bridge_path.to_string()),
            ("REQUEST_METHOD", "POST".to_string()),
            ("CONTENT_TYPE", "application/json".to_string()),
            ("CONTENT_LENGTH", body.len().to_string()),
            ("SERVER_PROTOCOL", "HTTP/1.1".to_string()),
            ("GATEWAY_INTERFACE", "CGI/1.1".to_string()),
            ("REQUEST_URI", "/".to_string()),
            ("DOCUMENT_ROOT", "/".to_string()),
            ("REMOTE_ADDR", "127.0.0.1".to_string()),
            ("REMOTE_PORT", "0".to_string()),
            ("SERVER_ADDR", "127.0.0.1".to_string()),
            ("SERVER_PORT", "80".to_string()),
            ("SERVER_NAME", "neutralts".to_string()),
        ];

        let mut params_buf = Vec::new();
        for (name, value) in params {
            Self::write_name_value_pair(&mut params_buf, name.as_bytes(), value.as_bytes())?;
        }

        if !params_buf.is_empty() {
            Self::write_record(stream, FCGI_PARAMS, FCGI_REQUEST_ID, &params_buf)?;
        }
        Self::write_record(stream, FCGI_PARAMS, FCGI_REQUEST_ID, &[])?;

        if !body.is_empty() {
            Self::write_record(stream, FCGI_STDIN, FCGI_REQUEST_ID, body)?;
        }
        Self::write_record(stream, FCGI_STDIN, FCGI_REQUEST_ID, &[])?;
        stream.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    fn read_fastcgi_response(stream: &mut FpmStream) -> Result<(String, String), String> {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        loop {
            let mut header = [0u8; 8];
            match stream.read_exact(&mut header) {
                Ok(_) => {}
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                    // Some FPM setups close the socket immediately after STDOUT/STDERR
                    // without sending END_REQUEST. If we already have payload, treat as done.
                    if !stdout.is_empty() || !stderr.is_empty() {
                        break;
                    }
                    return Err(e.to_string());
                }
                Err(e) => return Err(e.to_string()),
            }

            // FastCGI response must start with version 1.
            // If not, this is likely not a FastCGI endpoint (for example, HTTP server on :9000).
            if header[0] != FCGI_VERSION_1 {
                let prefix = String::from_utf8_lossy(&header);
                if prefix.starts_with("HTTP/") {
                    return Err(
                        "invalid FastCGI response: endpoint looks like HTTP, not PHP-FPM"
                            .to_string(),
                    );
                }
                return Err(format!(
                    "invalid FastCGI response: unsupported version {}",
                    header[0]
                ));
            }

            let record_type = header[1];
            let request_id = u16::from_be_bytes([header[2], header[3]]);
            let content_length = u16::from_be_bytes([header[4], header[5]]) as usize;
            let padding_length = header[6] as usize;

            let mut content = vec![0u8; content_length];
            if content_length > 0 {
                match stream.read_exact(&mut content) {
                    Ok(_) => {}
                    Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                        if !stdout.is_empty() || !stderr.is_empty() {
                            break;
                        }
                        return Err(e.to_string());
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
            if padding_length > 0 {
                let mut padding = vec![0u8; padding_length];
                match stream.read_exact(&mut padding) {
                    Ok(_) => {}
                    Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                        if !stdout.is_empty() || !stderr.is_empty() {
                            break;
                        }
                        return Err(e.to_string());
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }

            if request_id != FCGI_REQUEST_ID {
                continue;
            }

            match record_type {
                FCGI_STDOUT => stdout.extend_from_slice(&content),
                FCGI_STDERR => stderr.extend_from_slice(&content),
                FCGI_END_REQUEST => break,
                _ => {}
            }
        }

        Ok((
            String::from_utf8_lossy(&stdout).into_owned(),
            String::from_utf8_lossy(&stderr).into_owned(),
        ))
    }

    fn extract_http_body(stdout: &str) -> &str {
        if let Some(pos) = stdout.find("\r\n\r\n") {
            &stdout[pos + 4..]
        } else if let Some(pos) = stdout.find("\n\n") {
            &stdout[pos + 2..]
        } else {
            stdout
        }
    }

    fn write_record(
        stream: &mut FpmStream,
        record_type: u8,
        request_id: u16,
        content: &[u8],
    ) -> Result<(), String> {
        if content.len() > u16::MAX as usize {
            return Err("fastcgi content too large".to_string());
        }

        let padding_len = (8 - (content.len() % 8)) % 8;
        let header = [
            FCGI_VERSION_1,
            record_type,
            (request_id >> 8) as u8,
            (request_id & 0xFF) as u8,
            ((content.len() >> 8) & 0xFF) as u8,
            (content.len() & 0xFF) as u8,
            padding_len as u8,
            0,
        ];

        stream.write_all(&header).map_err(|e| e.to_string())?;
        if !content.is_empty() {
            stream.write_all(content).map_err(|e| e.to_string())?;
        }
        if padding_len > 0 {
            let padding = vec![0u8; padding_len];
            stream.write_all(&padding).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn write_name_value_pair(out: &mut Vec<u8>, name: &[u8], value: &[u8]) -> Result<(), String> {
        Self::write_length(out, name.len())?;
        Self::write_length(out, value.len())?;
        out.extend_from_slice(name);
        out.extend_from_slice(value);
        Ok(())
    }

    fn write_length(out: &mut Vec<u8>, len: usize) -> Result<(), String> {
        if len < 128 {
            out.push(len as u8);
            return Ok(());
        }
        if len > 0x7fff_ffff {
            return Err("fastcgi name/value too large".to_string());
        }
        out.push(((len >> 24) as u8) | 0x80);
        out.push((len >> 16) as u8);
        out.push((len >> 8) as u8);
        out.push(len as u8);
        Ok(())
    }
}
