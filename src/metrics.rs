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
            let min = self.latencies.iter().min().unwrap();
            let max = self.latencies.iter().max().unwrap();
            let sum: Duration = self.latencies.iter().sum();
            let mean = sum / self.latencies.len() as u32;

            println!("\nLatency:");
            println!("  Min:    {}", format_duration(*min));
            println!("  Max:    {}", format_duration(*max));
            println!("  Mean:   {}", format_duration(mean));
        }
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
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
