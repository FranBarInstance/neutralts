/// Performance test for large schema processing
/// Measures the time to create a Template with a large schema

#[cfg(not(debug_assertions))]
use std::time::Instant;

/// Generate a large schema with the specified number of keys
#[cfg(not(debug_assertions))]
fn generate_large_schema(num_keys: usize) -> serde_json::Value {
    let mut data = serde_json::Map::new();

    for i in 0..num_keys {
        let key = format!("key_{}", i);
        let value = serde_json::json!({
            "id": i,
            "name": format!("Item {}", i),
            "description": format!("This is a detailed description for item number {} with some extra text to make it larger", i),
            "tags": ["tag1", "tag2", "tag3"],
            "metadata": {
                "created": "2024-01-01",
                "updated": "2024-12-31",
                "author": format!("author_{}", i % 100),
                "version": format!("1.{}.0", i % 10)
            }
        });
        data.insert(key, value);
    }

    serde_json::json!({
        "config": {
            "infinite_loop_max_bifs": 555000,
            "comments": "keep",
            "errors": "hide"
        },
        "inherit": {
            "snippets": {},
            "declare": {
                "any": "*"
            },
            "params": {},
            "locale": {
                "current": "en",
                "trans": {}
            }
        },
        "data": data
    })
}

#[cfg(not(debug_assertions))]
#[test]
fn test_large_schema_performance() {
    let schema_sizes = [100, 500, 1000, 2000];
    let iterations = 10;

    println!("\n=== Large Schema Performance Test ===\n");
    println!("{:<10} {:<15} {:<15} {:<15}", "Keys", "Size (KB)", "Time (ms)", "Per key (µs)");
    println!("{}", "-".repeat(55));

    for num_keys in schema_sizes {
        let schema = generate_large_schema(num_keys);
        let schema_size = serde_json::to_string(&schema).unwrap().len();

        // Warmup
        let schema_clone = schema.clone();
        let _ = neutralts::Template::from_file_value("tests/obj.ntpl", schema_clone);

        // Measure
        let mut total_time = 0u128;
        for _ in 0..iterations {
            let schema_clone = schema.clone();
            let start = Instant::now();
            let result = neutralts::Template::from_file_value("tests/obj.ntpl", schema_clone);
            let elapsed = start.elapsed().as_nanos();

            assert!(result.is_ok(), "Template creation failed");
            total_time += elapsed;
        }

        let avg_time_ns = total_time / iterations as u128;
        let avg_time_ms = avg_time_ns as f64 / 1_000_000.0;
        let per_key_us = (avg_time_ns as f64 / num_keys as f64) / 1000.0;

        println!(
            "{:<10} {:<15.1} {:<15.2} {:<15.2}",
            num_keys,
            schema_size as f64 / 1024.0,
            avg_time_ms,
            per_key_us
        );
    }

    println!();
}

#[cfg(not(debug_assertions))]
#[test]
fn test_schema_merge_performance() {
    use neutralts::Template;

    let num_keys = 1000;
    let schema = generate_large_schema(num_keys);
    let schema_str = serde_json::to_string(&schema).unwrap();

    println!("\n=== Schema Merge Performance Test ===\n");
    println!("Schema size: {} bytes ({:.1} KB)", schema_str.len(), schema_str.len() as f64 / 1024.0);

    // Test merge_schema_str (parses JSON)
    let iterations = 20;
    let mut total_time = 0u128;

    for _ in 0..iterations {
        let mut template = Template::new().unwrap();
        let start = Instant::now();
        template.merge_schema_str(&schema_str).unwrap();
        total_time += start.elapsed().as_nanos();
    }

    let avg_time_ms = (total_time / iterations as u128) as f64 / 1_000_000.0;
    println!("merge_schema_str avg time: {:.2} ms", avg_time_ms);

    // Test merge_schema_value (already parsed)
    let mut total_time = 0u128;

    for _ in 0..iterations {
        let mut template = Template::new().unwrap();
        let schema_clone = schema.clone();
        let start = Instant::now();
        template.merge_schema_value(schema_clone);
        total_time += start.elapsed().as_nanos();
    }

    let avg_time_ms = (total_time / iterations as u128) as f64 / 1_000_000.0;
    println!("merge_schema_value avg time: {:.2} ms", avg_time_ms);

    println!();
}
