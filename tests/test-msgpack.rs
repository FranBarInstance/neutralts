use neutralts::Template;
use serde_json::json;
use std::fs;

#[test]
fn test_from_file_msgpack_and_merge_path() {
    let schema = json!({
        "data": {
            "name": "MessagePack Test",
            "status": "Online"
        }
    });

    // Serialize to MessagePack
    let msgpack_data = rmp_serde::to_vec(&schema).unwrap();
    let msgpack_path = "tests/test_data_api.msgpack";
    fs::write(msgpack_path, msgpack_data).unwrap();

    let template_path = "tests/test_msgpack_api.ntpl";
    fs::write(template_path, "Hello {:;name:}, Status: {:;status:}").unwrap();

    // The user strategy: from_file_msgpack with empty data, then merge_schema_msgpack_path
    let mut template = Template::from_file_msgpack(template_path, &[]).unwrap();
    template.merge_schema_msgpack_path(msgpack_path).unwrap();

    let result = template.render();

    assert_eq!(result, "Hello MessagePack Test, Status: Online");

    // Cleanup
    fs::remove_file(msgpack_path).unwrap();
    fs::remove_file(template_path).unwrap();
}

#[test]
fn test_from_file_msgpack_memory() {
    let schema = json!({
        "data": {
            "name": "Memory Test"
        }
    });

    let msgpack_data = rmp_serde::to_vec(&schema).unwrap();
    let template_path = "tests/test_msgpack_mem_api.ntpl";
    fs::write(template_path, "Hello {:;name:}").unwrap();

    let mut template = Template::from_file_msgpack(template_path, &msgpack_data).unwrap();
    let result = template.render();

    assert_eq!(result, "Hello Memory Test");

    // Cleanup
    fs::remove_file(template_path).unwrap();
}

#[test]
fn test_merge_schema_msgpack_path() {
    let mut template = Template::new().unwrap();

    let extra_data = json!({
        "data": {
            "extra": "Information"
        }
    });

    let msgpack_data = rmp_serde::to_vec(&extra_data).unwrap();
    let msgpack_path = "tests/test_merge_api.msgpack";
    fs::write(msgpack_path, msgpack_data).unwrap();

    template.merge_schema_msgpack_path(msgpack_path).unwrap();
    template.set_src_str("Extra: {:;extra:}");
    let result = template.render();

    assert_eq!(result, "Extra: Information");

    // Cleanup
    fs::remove_file(msgpack_path).unwrap();
}

#[test]
fn test_merge_schema_msgpack_memory() {
    let mut template = Template::new().unwrap();

    let extra_data = json!({
        "data": {
            "byte_data": "Worked"
        }
    });

    let msgpack_data = rmp_serde::to_vec(&extra_data).unwrap();
    template.merge_schema_msgpack(&msgpack_data).unwrap();

    template.set_src_str("Bytes: {:;byte_data:}");
    let result = template.render();

    assert_eq!(result, "Bytes: Worked");
}
