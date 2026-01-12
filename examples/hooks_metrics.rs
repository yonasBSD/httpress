use httpress::{Benchmark, HookAction, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Benchmark with custom metrics collection using hooks\n");

    // Define custom metrics structure
    #[derive(Default)]
    struct CustomMetrics {
        slow_requests: usize,        // Requests slower than 100ms
        very_slow_requests: usize,   // Requests slower than 500ms
        by_worker: HashMap<usize, usize>,
        status_codes: HashMap<u16, usize>,
    }

    let metrics = Arc::new(Mutex::new(CustomMetrics::default()));
    let metrics_clone = metrics.clone();

    // Run benchmark with after_request hook for metrics collection
    let results = Benchmark::builder()
        .url("http://localhost:3000")
        .concurrency(10)
        .requests(100)
        .after_request(move |ctx| {
            let mut m = metrics_clone.lock().unwrap();

            // Track slow requests
            if ctx.latency > Duration::from_millis(500) {
                m.very_slow_requests += 1;
            } else if ctx.latency > Duration::from_millis(100) {
                m.slow_requests += 1;
            }

            // Track requests per worker
            *m.by_worker.entry(ctx.worker_id).or_insert(0) += 1;

            // Track status code distribution
            if let Some(status) = ctx.status {
                *m.status_codes.entry(status).or_insert(0) += 1;
            }

            HookAction::Continue
        })
        .build()?
        .run()
        .await?;

    // Print built-in results
    results.print();

    // Print custom metrics
    let m = metrics.lock().unwrap();
    println!("\n--- Custom Metrics ---");
    println!("Slow requests (>100ms): {}", m.slow_requests);
    println!("Very slow requests (>500ms): {}", m.very_slow_requests);

    println!("\nRequests by worker:");
    let mut workers: Vec<_> = m.by_worker.iter().collect();
    workers.sort_by_key(|(id, _)| *id);
    for (worker_id, count) in workers {
        println!("  Worker {}: {} requests", worker_id, count);
    }

    println!("\nStatus code distribution:");
    let mut status_codes: Vec<_> = m.status_codes.iter().collect();
    status_codes.sort_by_key(|(code, _)| *code);
    for (status, count) in status_codes {
        println!("  {}: {} requests", status, count);
    }

    Ok(())
}
