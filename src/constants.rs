use std::collections::HashMap;
use lazy_static::lazy_static;

///  Bif delimiters
///
/// ```text
///   .------------------------------> BIF_OPEN = { + BIF_DELIM
///   |.-----------------------------> BIF_DELIM
///   ||.----------------------------> BIF_MOD_... (Modifiers)
///   |||       .--------------------> BIF_NAME
///   |||       |    .---------------> BIF_ARRAY
///   |||       |    ||    .---------> BIF_CODE
///   |||       |    ||    ||      .-> BIF_CLOSE = BIF_DELIM + }
///   |||       |    ||    ||      |
///   vvv       v    vv    vv      v
///   {:!snippet; var->key >> ... :}
/// ```
pub const BIF_DELIM: &str = ":";

///  Bif delimiters
///
/// ```text
///   .------------------------------> BIF_OPEN = { + BIF_DELIM
///   |.-----------------------------> BIF_DELIM
///   ||.----------------------------> BIF_MOD_... (Modifiers)
///   |||       .--------------------> BIF_NAME
///   |||       |    .---------------> BIF_ARRAY
///   |||       |    ||    .---------> BIF_CODE
///   |||       |    ||    ||      .-> BIF_CLOSE = BIF_DELIM + }
///   |||       |    ||    ||      |
///   vvv       v    vv    vv      v
///   {:!snippet; var->key >> ... :}
/// ```
pub const BIF_OPEN: &str = "{:";

///  Bif delimiters
///
/// ```text
///   .------------------------------> BIF_OPEN = { + BIF_DELIM
///   |.-----------------------------> BIF_DELIM
///   ||.----------------------------> BIF_MOD_... (Modifiers)
///   |||       .--------------------> BIF_NAME
///   |||       |    .---------------> BIF_ARRAY
///   |||       |    ||    .---------> BIF_CODE
///   |||       |    ||    ||      .-> BIF_CLOSE = BIF_DELIM + }
///   |||       |    ||    ||      |
///   vvv       v    vv    vv      v
///   {:!snippet; var->key >> ... :}
/// ```
pub const BIF_CLOSE: &str = ":}";

///  Bif delimiters
///
/// ```text
///   .------------------------------> BIF_OPEN = { + BIF_DELIM
///   |.-----------------------------> BIF_DELIM
///   ||.----------------------------> BIF_MOD_... (Modifiers)
///   |||       .--------------------> BIF_NAME
///   |||       |    .---------------> BIF_ARRAY
///   |||       |    ||    .---------> BIF_CODE
///   |||       |    ||    ||      .-> BIF_CLOSE = BIF_DELIM + }
///   |||       |    ||    ||      |
///   vvv       v    vv    vv      v
///   {:!snippet; var->key >> ... :}
/// ```
pub const BIF_NAME: &str = ";";

///  Bif delimiters
///
/// ```text
///   .------------------------------> BIF_OPEN = { + BIF_DELIM
///   |.-----------------------------> BIF_DELIM
///   ||.----------------------------> BIF_MOD_... (Modifiers)
///   |||       .--------------------> BIF_NAME
///   |||       |    .---------------> BIF_ARRAY
///   |||       |    ||    .---------> BIF_CODE
///   |||       |    ||    ||      .-> BIF_CLOSE = BIF_DELIM + }
///   |||       |    ||    ||      |
///   vvv       v    vv    vv      v
///   {:!snippet; var->key >> ... :}
/// ```
pub const BIF_CODE: &str = ">>";

///  Bif delimiters
///
/// ```text
///   .------------------------------> BIF_OPEN = { + BIF_DELIM
///   |.-----------------------------> BIF_DELIM
///   ||.----------------------------> BIF_MOD_... (Modifiers)
///   |||       .--------------------> BIF_NAME
///   |||       |    .---------------> BIF_ARRAY
///   |||       |    ||    .---------> BIF_CODE
///   |||       |    ||    ||      .-> BIF_CLOSE = BIF_DELIM + }
///   |||       |    ||    ||      |
///   vvv       v    vv    vv      v
///   {:!snippet; var->key >> ... :}
/// ```
pub const BIF_ARRAY: &str = "->";

///  Bif comment
///
/// ```text
///      .---> BIF_COMMENT
///     |
///     v
///   {:* commnet *:}
/// ```
pub const BIF_COMMENT: &str = "*";

///  Bif comment open
///
/// ```text
///   .---> BIF_COMMENT_OPEN
///   |||
///   vvv
///   {:* commnet *:}
/// ```
pub const BIF_COMMENT_OPEN: &str = "{:*";

///  Bif comment close
///
/// ```text
///               .---> BIF_COMMENT_CLOSE
///               |||
///               vvv
///   {:* commnet *:}
/// ```
pub const BIF_COMMENT_CLOSE: &str = "*:}";

/// modifier filter
pub const BIF_MOD_FILTER: &str = "&";

/// modifier not
pub const BIF_MOD_NEGATE: &str = "!";

/// modifier upline
pub const BIF_MOD_UPLINE: &str = "^";

/// modifier scope
pub const BIF_MOD_SCOPE: &str = "+";

/// Identify bif var as it has no name
pub const BIF_VAR: &str = "{:;";

/// delim as bytes
pub const BIF_DELIM_B: &[u8] = BIF_DELIM.as_bytes();

/// BIF_OPEN as bytes
pub const BIF_OPEN_B: &[u8] = BIF_OPEN.as_bytes();

/// BIF_OPEN 0 as bytes
pub const BIF_OPEN0: u8 = BIF_OPEN.as_bytes()[0];

/// BIF_OPEN 1 as bytes
pub const BIF_OPEN1: u8 = BIF_OPEN.as_bytes()[1];

/// BIF_CLOSE as bytes
pub const BIF_CLOSE_B: &[u8] = BIF_CLOSE.as_bytes();

/// BIF_CLOSE 0 as bytes
pub const BIF_CLOSE0: u8 = BIF_CLOSE.as_bytes()[0];

/// BIF_CLOSE 1 as bytes
pub const BIF_CLOSE1: u8 = BIF_CLOSE.as_bytes()[1];

/// BIF_CODE as bytes
pub const BIF_CODE_B: &[u8] = BIF_CODE.as_bytes();

/// BIF_COMMENT as bytes
pub const BIF_COMMENT_B: u8 = BIF_COMMENT.as_bytes()[0];

/// BIF_COMMENT_OPEN as bytes
pub const BIF_COMMENT_OPEN_B: &[u8] = BIF_COMMENT_OPEN.as_bytes();

/// BIF_COMMENT_CLOSE as bytes
pub const BIF_COMMENT_CLOSE_B: &[u8] = BIF_COMMENT_CLOSE.as_bytes();

/// replacement for BIF_OPEN sanitation
pub const BIF_SANITIZE_OPEN: &str = "&#123;:";

/// replacement for BIF_CLOSE sanitation
pub const BIF_SANITIZE_CLOSE: &str = ":&#125;";

/// html entity for {:;:}
pub const UNPRINTABLE: &str = "&#0;";

/// html entity for null
pub const NULL: &str = "&#0;";

/// empty
pub const EMPTY: &str = "";

/// Empty string could be different in different environments and could be an
/// HTM entity. It is also used to make it clear that it is an empty string.
pub const EMPTY_STRING: String = String::new();

/// html entity for space
pub const SPACE: &str = "&#160;";

/// html entity for crlf
pub const CRLF: &str = "&#10;";

/// html entity for Backspace
pub const BACKSPACE: &str = "&#9224";

/// false
pub const FALSE: bool = false;

/// true
pub const TRUE: bool = true;

/// Identify bif allow
pub const BIF_ALLOWED: [&str; 2] = ["{:allow;", "{:!allow;"];

/// To detect the template files that may contain snippet
pub const SNIPPETS_FILES: &str = "snippet";

/// bif list
pub const BIF_LIST: [&str; 36] = [
    "",
    "allow",
    "array",
    "bool",
    "cache",
    "coalesce",
    "code",
    "contains",
    "count",
    "data",
    "date",
    "declare",
    "defined",
    "each",
    "else",
    "eval",
    "exit",
    "fetch",
    "filled",
    "flg",
    "for",
    "hash",
    "include",
    "join",
    "lang",
    "locale",
    "moveto",
    "neutral",
    "param",
    "rand",
    "redirect",
    "replace",
    "same",
    "snippet",
    "sum",
    "trans",
];

/// bif alias list because some bifs have no name
pub const BIF_ALIAS_LIST: [&str; 37] = [
    "allow",
    "array",
    "bool",
    "cache",
    "coalesce",
    "code",
    "contains",
    "count",
    "data",
    "date",
    "declare",
    "defined",
    "each",
    "else",
    "eval",
    "exit",
    "fetch",
    "filled",
    "flg",
    "for",
    "hash",
    "include",
    "join",
    "lang",
    "locale",
    "moveto",
    "neutral",
    "param",
    "rand",
    "redirect",
    "replace",
    "same",
    "snippet",
    "sum",
    "trans",
    "unprintable",
    "var",
];

lazy_static! {
    /// HTTP status codes
    pub static ref STATUS_CODES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("100", "Continue");
        m.insert("101", "Switching Protocols");
        m.insert("103", "Early Hints");
        m.insert("200", "OK");
        m.insert("201", "Created");
        m.insert("202", "Accepted");
        m.insert("203", "Non-Authoritative Information");
        m.insert("204", "No Content");
        m.insert("205", "Reset Content");
        m.insert("206", "Partial Content");
        m.insert("208", "Already Reported");
        m.insert("226", "IM Used");
        m.insert("300", "Multiple Choices");
        m.insert("301", "Moved Permanently");
        m.insert("302", "Found");
        m.insert("303", "See Other");
        m.insert("304", "Not Modified");
        m.insert("305", "Use Proxy");
        m.insert("306", "Switch Proxy"); // old http version
        m.insert("307", "Temporary Redirect");
        m.insert("308", "Permanent Redirect");
        m.insert("400", "Bad Request");
        m.insert("401", "Unauthorized");
        m.insert("402", "Payment Required");
        m.insert("403", "Forbidden");
        m.insert("404", "Not Found");
        m.insert("405", "Method Not Allowed");
        m.insert("406", "Not Acceptable");
        m.insert("407", "Proxy Authentication Required");
        m.insert("408", "Request Time-out");
        m.insert("409", "Conflict");
        m.insert("410", "Gone");
        m.insert("411", "Length Required");
        m.insert("412", "Precondition Failed");
        m.insert("413", "Payload Too Large");
        m.insert("414", "URI Too Long");
        m.insert("415", "Unsupported Media Type");
        m.insert("416", "Range Not Satisfiable");
        m.insert("417", "Expectation Failed");
        m.insert("421", "Misdirected Request");
        m.insert("422", "Unprocessable Entity");
        m.insert("423", "Locked");
        m.insert("424", "Failed Dependency");
        m.insert("425", "Too Early");
        m.insert("426", "Upgrade Required");
        m.insert("428", "Precondition Required");
        m.insert("429", "Too Many Requests");
        m.insert("431", "Request Header Fields Too Large");
        m.insert("451", "Unavailable For Legal Reasons");
        m.insert("500", "Internal Server Error");
        m.insert("501", "Not Implemented");
        m.insert("502", "Bad Gateway");
        m.insert("503", "Service Unavailable");
        m.insert("504", "Gateway Time-out");
        m.insert("505", "HTTP Version Not Supported");
        m.insert("506", "Variant Also Negotiates (Experimental)");
        m.insert("510", "Not Extended");
        m.insert("511", "Network Authentication Required");

        m
    };
}

/// JavaScript script reloads the current top page, used in {:redirect;
pub const REDIR_JS_RELOAD_TOP: &str = "<!DOCTYPE html><script>top.location.href=self.location.href.split('#')[0];</script>";

/// JavaScript script reloads the current, used in {:redirect;
pub const REDIR_JS_RELOAD_SELF: &str = "<!DOCTYPE html><script>self.location.href=self.location.href.split('#')[0]</script>";

/// JavaScript script that redirects to a new URL in the top page, used in {:redirect;
/// The placeholder '{}' should be replaced with the destination url.
pub const REDIR_JS_REDIRECT_TOP: &str = "<!DOCTYPE html><script>top.location.href='{}';</script>";

/// JavaScript script that redirects to a new URL in the self page, used in {:redirect;
/// The placeholder '{}' should be replaced with the destination url.
pub const REDIR_JS_REDIRECT_SELF: &str = "<!DOCTYPE html><script>self.location.href='{}';</script>";

lazy_static! {
    /// JavaScript for {:fetch; ... :}
    pub static ref NEUTRAL_JS: String = r#"<script>{
        const d=document;
        let sl='...';
        let sd=250;
        let st=30000;
        let se=' Form ERROR! ';
        let sed=3500;
        "undefined"!=typeof neutral_submit_loading&&(sl=neutral_submit_loading);
        "undefined"!=typeof neutral_submit_delay&&(sd=neutral_submit_delay);
        "undefined"!=typeof neutral_submit_timeout&&(st=neutral_submit_timeout);
        "undefined"!=typeof neutral_submit_error&&(se=neutral_submit_error);
        "undefined"!=typeof neutral_submit_error_delay&&(sed=neutral_submit_error_delay);

        const o=new IntersectionObserver((e)=>{
            e.forEach(e=>{
                if(e.isIntersecting){
                    o.unobserve(e.target);
                    neutral_fetch(e.target,e.target.dataset.url,e.target.dataset.wrap);
                }
            });
        });

        function neutral_obs(){
            d.querySelectorAll('.neutral-fetch-visible').forEach((e)=>{
                e.classList.remove('neutral-fetch-visible');
                o.observe(e);
            });
        }

        function neutral_fetch(e,u,w){
            let h=e;if(w){h=d.getElementById(w)}
            fetch(u,{headers:{'requested-with-ajax':'fetch'}})
            .then(response=>response.text())
            .then(html=>{
                h.innerHTML=html;
                h.outerHTML=html;
                window.dispatchEvent(new CustomEvent('neutralFetchCompleted',{detail:{element:h,url:u}}));
            })
            .catch(error=>{
                h.innerHTML=se;
                window.dispatchEvent(new CustomEvent('neutralFetchError',{detail:{element:h,url:u}}));
            });
        }

        function neutral_fetch_form(f,u,w){
            const subs = f.querySelectorAll('[type=submit]');
            subs.forEach((e)=>{
                e.disabled=true;
                e.dataset['restsubs']=e.innerHTML;
                e.innerHTML=e.innerHTML + ' ' + sl;
            });
            let h=f;if(w){h = d.getElementById(w)}
            let dt=new FormData(f);
            let has_file=false;
            for(let i=0;i<f.elements.length;i++){
                value=f.elements[i].value;
                if(f.elements[i].name===""){
                    continue;
                }
                if(f.elements[i].type=="file"){
                    has_file=true;
                    continue;
                }
                if(f.elements[i].name.indexOf('[]')>=0){
                    continue;
                }
                if(f.elements[i].type=="checkbox" || f.elements[i].type=="radio"){
                    if(!f.elements[i].checked){
                        continue;
                    }
                }
                dt.append(f.elements[i].name,value);
            }
            if(!has_file){dt=new URLSearchParams(new FormData(f))}
            setTimeout(()=>{
                fetch(u,{
                    signal:AbortSignal.timeout(st),
                    method:'POST',
                    headers:{'requested-with-ajax':'fetch'},
                    body:dt,
                })
                .then(response=>response.text())
                .then(html=>{
                    h.innerHTML=html;
                    h.outerHTML=html;
                    window.dispatchEvent(new CustomEvent('neutralFetchCompleted',{detail:{element:h,url:u}}));
                })
                .catch(error=>{
                    window.dispatchEvent(new CustomEvent('neutralFetchError',{detail:{element:h,url:u}}));
                    subs.forEach((e)=>{e.disabled=true;e.innerHTML=se;});
                    setTimeout(()=>{
                        subs.forEach((e)=>{e.disabled=false;e.innerHTML=e.dataset['restsubs']});
                    },sed);
                });
            },sd);
        }

        function neutral_fev(){
            d.querySelectorAll('.neutral-fetch-form').forEach((e)=>{
                e.classList.remove('neutral-fetch-form');
                function handleSubmit(ev){
                    ev.preventDefault();
                    neutral_fetch_form(e,e.getAttribute('action'),e.dataset.wrap);
                }
                e.addEventListener('submit',handleSubmit);
            });

            d.querySelectorAll('.neutral-fetch-auto').forEach((e)=>{
                e.classList.remove('neutral-fetch-auto');
                neutral_fetch(e,e.dataset.url,e.dataset.wrap);
            });

            d.querySelectorAll('.neutral-fetch-click').forEach((e)=>{
                e.classList.remove('neutral-fetch-click');
                function handleClick(){
                    neutral_fetch(e,e.dataset.url,e.dataset.wrap);
                }
                e.addEventListener('click',handleClick);
            });
            neutral_obs();
        }

        d.addEventListener('DOMContentLoaded',()=>{
            neutral_fev();
        });
        window.addEventListener('neutralFetchCompleted',()=>{
            neutral_fev();
        });
        window.addEventListener('neutralFetchError',()=>{
            neutral_fev();
        });
    }</script>"#.replace("\n", "").replace("  ", "");
}

/// HTML container form for {:fetch; ... :}
pub const DIV_FETCH_FORM: &str = r#"
<form id="{id}" name="{name}" class="neutral-fetch-form {class}" method="POST" action="{endpoint}" data-wrap="{wrap}">
    {body}
</form>
"#;

/// HTML container none for {:fetch; ... :}
pub const DIV_FETCH_NONE: &str = r#"
<div id="{id}" class="neutral-fetch-none {class}" data-url="{endpoint}" data-wrap="{wrap}">
    {body}
</div>
"#;

/// HTML container auto for {:fetch; ... :}
pub const DIV_FETCH_AUTO: &str = r#"
<div id="{id}" class="neutral-fetch-auto {class}" data-url="{endpoint}" data-wrap="{wrap}">
    {body}
</div>
"#;

/// HTML container visible for {:fetch; ... :}
pub const DIV_FETCH_VISIBLE: &str = r#"
<div id="{id}" class="neutral-fetch-visible {class}" data-url="{endpoint}" data-wrap="{wrap}">
    {body}
</div>
"#;

/// HTML container click for {:fetch; ... :}
pub const DIV_FETCH_CLICK: &str = r#"
<div id="{id}" class="neutral-fetch-click {class}" data-url="{endpoint}" data-wrap="{wrap}">
    {body}
</div>
"#;

pub const DEFAULT_OBJ_ENGINE: &str = "python";
pub const DEFAULT_OBJ_CALLBACK: &str = "main";
