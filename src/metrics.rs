use std::time::Duration;

/// Result of a single HTTP request
pub struct RequestResult {
    pub latency: Duration,
    pub success: bool,
}

/// Aggregated metrics from all requests
pub struct Metrics {
    pub total: usize,
    pub success: usize,
    pub latencies: Vec<Duration>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            total: 0,
            success: 0,
            latencies: Vec::new(),
        }
    }

    pub fn record(&mut self, result: RequestResult) {
        self.total += 1;
        if result.success {
            self.success += 1;
        }
        self.latencies.push(result.latency);
    }

    pub fn report(&self, elapsed: Duration) {
        let errors = self.total - self.success;
        let throughput = self.total as f64 / elapsed.as_secs_f64();

        println!("\n--- Benchmark Complete ---");
        println!(
            "Requests:     {} total, {} success, {} errors",
            self.total, self.success, errors
        );
        println!("Duration:     {:.2}s", elapsed.as_secs_f64());
        println!("Throughput:   {:.2} req/s", throughput);

        if !self.latencies.is_empty() {
            let mut sorted = self.latencies.clone();
            sorted.sort();

            let min = sorted.first().unwrap();
            let max = sorted.last().unwrap();
            let sum: Duration = sorted.iter().sum();
            let mean = sum / sorted.len() as u32;

            let p50 = percentile(&sorted, 50);
            let p90 = percentile(&sorted, 90);
            let p95 = percentile(&sorted, 95);
            let p99 = percentile(&sorted, 99);

            println!("\nLatency:");
            println!("  Min:    {}", format_duration(*min));
            println!("  Max:    {}", format_duration(*max));
            println!("  Mean:   {}", format_duration(mean));
            println!("  p50:    {}", format_duration(p50));
            println!("  p90:    {}", format_duration(p90));
            println!("  p95:    {}", format_duration(p95));
            println!("  p99:    {}", format_duration(p99));
        }
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

fn percentile(sorted: &[Duration], p: usize) -> Duration {
    let idx = (sorted.len() * p / 100).saturating_sub(1).max(0);
    sorted[idx]
}

fn format_duration(d: Duration) -> String {
    let micros = d.as_micros();
    if micros < 1000 {
        format!("{}us", micros)
    } else if micros < 1_000_000 {
        format!("{:.2}ms", micros as f64 / 1000.0)
    } else {
        format!("{:.2}s", d.as_secs_f64())
    }
}
