//! Demonstration of the LLM comparison framework
//!
//! This example shows how to use the comparison engine to validate
//! rust2mojo output against LLM-generated code.

use rust2mojo::comparison::{BatchComparison, ComparisonConfig, ComparisonEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the comparison engine
    let config = ComparisonConfig {
        enabled: true,
        model: "claude-3-sonnet".to_string(),
        api_endpoint: "https://api.anthropic.com/v1/messages".to_string(),
        max_tokens: 4096,
        temperature: 0.1,
    };

    println!("🔍 Rust2Mojo LLM Comparison Demo");
    println!("================================\n");

    // Single comparison example
    println!("📋 Single Comparison Example");
    println!("-----------------------------");

    let engine = ComparisonEngine::new(config.clone());
    let rust_code = r#"
        fn fibonacci(n: u32) -> u32 {
            if n <= 1 {
                n
            } else {
                fibonacci(n - 1) + fibonacci(n - 2)
            }
        }
    "#;

    match engine.compare(rust_code).await {
        Ok(result) => {
            println!("✅ Comparison completed successfully!");
            println!(
                "Overall similarity score: {:.2}%",
                result.metrics.overall_score * 100.0
            );

            // Generate and display report
            let report = engine.generate_report(&result);
            println!("\n📊 Detailed Report:");
            println!("{}", report);
        }
        Err(e) => {
            println!("❌ Comparison failed: {}", e);
            println!("Note: This demo uses placeholder LLM integration.");
        }
    }

    // Batch comparison example
    println!("\n📚 Batch Comparison Example");
    println!("----------------------------");

    let mut batch = BatchComparison::new(config);

    let test_cases = [
        "fn add(a: i32, b: i32) -> i32 { a + b }",
        "fn factorial(n: u32) -> u32 { if n <= 1 { 1 } else { n * factorial(n - 1) } }",
        "struct Point { x: f64, y: f64 }",
    ];

    for (i, test_case) in test_cases.iter().enumerate() {
        println!(
            "Processing test case {}: {}",
            i + 1,
            test_case.lines().next().unwrap_or("")
        );
        match batch.add_test_case(test_case).await {
            Ok(()) => println!("  ✅ Added successfully"),
            Err(e) => println!("  ❌ Failed to add: {}", e),
        }
    }

    // Generate batch statistics
    let stats = batch.generate_statistics();
    println!("\n📈 Batch Statistics:");
    println!("  Total test cases: {}", stats.total_test_cases);
    println!(
        "  Average overall score: {:.2}%",
        stats.average_overall_score * 100.0
    );
    println!(
        "  Average structural similarity: {:.2}%",
        stats.average_structural_similarity * 100.0
    );

    // Generate full batch report (in real usage, you might save this to a file)
    let batch_report = batch.generate_batch_report();
    println!(
        "\n💾 Full batch report generated ({} characters)",
        batch_report.len()
    );

    println!("\n🎯 Demo completed!");
    println!("In a real implementation, this would:");
    println!("  • Make actual API calls to LLM services");
    println!("  • Perform sophisticated semantic analysis");
    println!("  • Generate performance benchmarks");
    println!("  • Provide actionable improvement suggestions");

    Ok(())
}
