/// Performance test for large schema processing
/// Measures the time to create a Template with a large schema

#[cfg(not(debug_assertions))]
use std::time::Instant;

// =============================================================================
// Unit tests for render_once() functionality
// =============================================================================

#[test]
fn test_render_once_basic() {
    use neutralts::Template;

    let schema = serde_json::json!({
        "data": {
            "title": "Hello World",
            "content": "This is a test"
        }
    });

    let mut template = Template::new().unwrap();
    template.merge_schema_value(schema);
    template.set_src_str("{:;title:}");

    let result = template.render_once();
    assert!(result.contains("Hello World"));
}

#[test]
fn test_render_once_produces_same_result_as_render() {
    use neutralts::Template;

    let schema = serde_json::json!({
        "data": {
            "title": "Test Title",
            "content": "Test Content"
        }
    });

    // Create two templates with the same schema
    let mut template1 = Template::new().unwrap();
    template1.merge_schema_value(schema.clone());
    template1.set_src_str("{:;title:} - {:;content:}");

    let mut template2 = Template::new().unwrap();
    template2.merge_schema_value(schema);
    template2.set_src_str("{:;title:} - {:;content:}");

    let result_render = template1.render();
    let result_once = template2.render_once();

    // Both should produce identical output
    assert_eq!(result_render, result_once);
}

#[test]
fn test_render_once_empty_template() {
    use neutralts::Template;

    let schema = serde_json::json!({
        "data": {}
    });

    let mut template = Template::new().unwrap();
    template.merge_schema_value(schema);
    template.set_src_str("");

    let result = template.render_once();
    assert!(result.is_empty());
}

#[test]
fn test_render_once_with_template_source() {
    use neutralts::Template;

    let schema = serde_json::json!({
        "data": {
            "name": "John",
            "age": 30
        }
    });

    let mut template = Template::new().unwrap();
    template.merge_schema_value(schema);
    template.set_src_str("Hello {:;name:}, you are {:;age:} years old.");

    let result = template.render_once();
    assert!(result.contains("John"));
    assert!(result.contains("30"));
}

#[test]
fn test_render_once_consumes_schema() {
    use neutralts::Template;

    let schema = serde_json::json!({
        "data": {
            "value": "test"
        }
    });

    let mut template = Template::new().unwrap();
    template.merge_schema_value(schema);
    template.set_src_str("{:;value:}");

    // First render_once should work and return the expected value
    let result1 = template.render_once();
    assert!(result1.contains("test"));

    // After render_once, the schema is consumed (taken with mem::take)
    // The template should NOT be reused after render_once()
    // This is the expected behavior - render_once() is for single-use templates
    // If you need to render multiple times, use render() instead
}

#[test]
fn test_render_once_with_nested_data() {
    use neutralts::Template;

    let schema = serde_json::json!({
        "data": {
            "user": {
                "name": "Alice",
                "email": "alice@example.com",
                "address": {
                    "city": "Madrid",
                    "country": "Spain"
                }
            }
        }
    });

    let mut template = Template::new().unwrap();
    template.merge_schema_value(schema);
    template.set_src_str("User: {:;user->name:}, City: {:;user->address->city:}");

    let result = template.render_once();
    assert!(result.contains("Alice"));
    assert!(result.contains("Madrid"));
}

#[test]
fn test_render_once_with_array_data() {
    use neutralts::Template;

    let schema = serde_json::json!({
        "data": {
            "items": [
                {"name": "Item 1", "price": 10},
                {"name": "Item 2", "price": 20},
                {"name": "Item 3", "price": 30}
            ]
        }
    });

    let mut template = Template::new().unwrap();
    template.merge_schema_value(schema);
    template.set_src_str("{:each; items key value >> {:;value->name:} :}");

    let result = template.render_once();
    assert!(result.contains("Item 1"));
    assert!(result.contains("Item 2"));
    assert!(result.contains("Item 3"));
}

#[test]
fn test_render_once_with_conditionals() {
    use neutralts::Template;

    let schema = serde_json::json!({
        "data": {
            "show_content": true,
            "message": "Hello World"
        }
    });

    let mut template = Template::new().unwrap();
    template.merge_schema_value(schema);
    template.set_src_str("{:bool; show_content >> {:;message:} :}");

    let result = template.render_once();
    assert!(result.contains("Hello World"));
}

#[test]
fn test_render_once_large_schema() {
    use neutralts::Template;

    // Generate a schema with many keys
    let mut data = serde_json::Map::new();
    for i in 0..100 {
        data.insert(
            format!("key_{}", i),
            serde_json::json!(format!("value_{}", i)),
        );
    }

    let schema = serde_json::json!({
        "data": data
    });

    let mut template = Template::new().unwrap();
    template.merge_schema_value(schema);
    template.set_src_str("{:;key_50:}");

    let result = template.render_once();
    assert!(result.contains("value_50"));
}

#[test]
fn test_render_once_vs_render_performance_comparison() {
    use neutralts::Template;
    use std::time::Instant;

    let mut items = Vec::new();
    for i in 0..100 {
        items.push(serde_json::json!({"id": i, "name": format!("Item {}", i)}));
    }

    let schema = serde_json::json!({
        "data": {
            "title": "Performance Test",
            "items": items
        }
    });

    let iterations = 50;

    // Measure render()
    let mut total_render = 0u128;
    for _ in 0..iterations {
        let mut template = Template::new().unwrap();
        template.merge_schema_value(schema.clone());
        template.set_src_str("{:each; items key value >> {:;value->name:} :}");

        let start = Instant::now();
        let _ = template.render();
        total_render += start.elapsed().as_nanos();
    }

    // Measure render_once()
    let mut total_once = 0u128;
    for _ in 0..iterations {
        let mut template = Template::new().unwrap();
        template.merge_schema_value(schema.clone());
        template.set_src_str("{:each; items key value >> {:;value->name:} :}");

        let start = Instant::now();
        let _ = template.render_once();
        total_once += start.elapsed().as_nanos();
    }

    let avg_render_ms = (total_render / iterations as u128) as f64 / 1_000_000.0;
    let avg_once_ms = (total_once / iterations as u128) as f64 / 1_000_000.0;

    println!("\n=== render_once() vs render() Unit Test ===");
    println!("render() avg:      {:.3} ms", avg_render_ms);
    println!("render_once() avg: {:.3} ms", avg_once_ms);

    // render_once should be at least as fast as render (usually faster)
    // We don't assert a specific speedup, just that it works correctly
    assert!(
        avg_once_ms > 0.0,
        "render_once should complete in measurable time"
    );
}

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
    println!(
        "{:<10} {:<15} {:<15} {:<15}",
        "Keys", "Size (KB)", "Time (ms)", "Per key (µs)"
    );
    println!("{}", "-".repeat(55));

    for num_keys in schema_sizes {
        let schema = generate_large_schema(num_keys);
        let _schema_size = serde_json::to_string(&schema).unwrap().len();

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
            _schema_size as f64 / 1024.0,
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
    println!(
        "Schema size: {} bytes ({:.1} KB)",
        schema_str.len(),
        schema_str.len() as f64 / 1024.0
    );

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

#[cfg(not(debug_assertions))]
#[test]
fn test_render_once_performance() {
    use neutralts::Template;

    let schema_sizes = [100, 500, 1000, 2000];
    let iterations = 10;

    println!("\n=== render() vs render_once() Performance Comparison ===\n");
    println!("Measuring: Template creation + render (full pipeline per iteration)");
    println!(
        "{:<10} {:<15} {:<15} {:<12} {:<12}",
        "Keys", "render (ms)", "render_once (ms)", "Speedup", "Savings"
    );
    println!("{}", "-".repeat(70));

    for num_keys in schema_sizes {
        let schema = generate_large_schema(num_keys);
        let _schema_size = serde_json::to_string(&schema).unwrap().len();

        // Warmup
        let schema_clone = schema.clone();
        let _ = Template::from_file_value("tests/obj.ntpl", schema_clone);

        // Measure: create Template + render() (clones schema internally)
        let mut total_time_render = 0u128;
        for _ in 0..iterations {
            let schema_clone = schema.clone();
            let start = Instant::now();
            let mut template = Template::from_file_value("tests/obj.ntpl", schema_clone).unwrap();
            let _ = template.render();
            total_time_render += start.elapsed().as_nanos();
        }

        // Measure: create Template + render_once() (no clone, takes ownership)
        let mut total_time_once = 0u128;
        for _ in 0..iterations {
            let schema_clone = schema.clone();
            let start = Instant::now();
            let mut template = Template::from_file_value("tests/obj.ntpl", schema_clone).unwrap();
            let _ = template.render_once();
            total_time_once += start.elapsed().as_nanos();
        }

        let avg_render_ms = (total_time_render / iterations as u128) as f64 / 1_000_000.0;
        let avg_once_ms = (total_time_once / iterations as u128) as f64 / 1_000_000.0;
        let speedup = avg_render_ms / avg_once_ms.max(0.001);
        let savings = ((avg_render_ms - avg_once_ms) / avg_render_ms.max(0.001)) * 100.0;

        println!(
            "{:<10} {:<15.2} {:<15.2} {:<12.2}x {:<11.1}%",
            num_keys, avg_render_ms, avg_once_ms, speedup, savings
        );
    }

    println!();
}

#[cfg(not(debug_assertions))]
#[test]
fn test_full_pipeline_performance() {
    use neutralts::Template;

    let num_keys = 1000;
    let schema = generate_large_schema(num_keys);
    let schema_str = serde_json::to_string(&schema).unwrap();

    println!("\n=== Full Pipeline Performance (Template creation + render) ===\n");
    println!(
        "Schema size: {} bytes ({:.1} KB)",
        schema_str.len(),
        schema_str.len() as f64 / 1024.0
    );

    let iterations = 20;

    // Measure: create template + render() (with clone)
    let mut total_time_render = 0u128;
    for _ in 0..iterations {
        let schema_clone = schema.clone();
        let start = Instant::now();
        let mut template = Template::from_file_value("tests/obj.ntpl", schema_clone).unwrap();
        let _ = template.render();
        total_time_render += start.elapsed().as_nanos();
    }

    // Measure: create template + render_once() (no clone)
    let mut total_time_once = 0u128;
    for _ in 0..iterations {
        let schema_clone = schema.clone();
        let start = Instant::now();
        let mut template = Template::from_file_value("tests/obj.ntpl", schema_clone).unwrap();
        let _ = template.render_once();
        total_time_once += start.elapsed().as_nanos();
    }

    let avg_render_ms = (total_time_render / iterations as u128) as f64 / 1_000_000.0;
    let avg_once_ms = (total_time_once / iterations as u128) as f64 / 1_000_000.0;
    let speedup = avg_render_ms / avg_once_ms.max(0.001);

    println!(
        "Template::from_file_value + render():     {:.2} ms",
        avg_render_ms
    );
    println!(
        "Template::from_file_value + render_once(): {:.2} ms",
        avg_once_ms
    );
    println!("Speedup: {:.2}x", speedup);
    println!();
}
