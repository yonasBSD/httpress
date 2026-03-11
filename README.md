<div align="center">

# httpress

[![Crates.io](https://img.shields.io/crates/v/httpress.svg)](https://crates.io/crates/httpress)
[![Documentation](https://docs.rs/httpress/badge.svg)](https://docs.rs/httpress)
[![License](https://img.shields.io/crates/l/httpress.svg)](https://github.com/TecuceanuGabriel/httpress)

A fast, flexible HTTP benchmarking library and CLI tool built in Rust.

[Features](#features) • [Performance](#performance) • [Installation](#installation) • [Quick Start](#quick-start) • [Library Usage](#library-usage) • [CLI Usage](#cli-usage) • [Examples](#examples)

</div>

## Demo

![demo](/assets/demo.gif)

## Features

- **Simple Builder API** - Fluent, type-safe configuration
- **High Performance** - Async Rust with minimal overhead
- **Flexible Rate Control** - Fixed rates or dynamic rate functions
- **Custom Request Generation** - Generate requests dynamically per-worker
- **Hook System** - Inject custom logic before/after requests
- **Detailed Metrics** - Latency percentiles, throughput, status codes
- **Concurrent Workers** - Configurable parallelism
- **Adaptive Testing** - Duration-based or request-count-based
- **Retry Logic** - Smart retry with hook-based control
- **Library + CLI** - Use as Rust library or standalone tool

## Performance

Benchmarks run against a local [hyper](https://hyper.rs/) echo server on loopback (`localhost`).
All tests use a 10 s duration. Environment: Intel Core i5-8350U, Arch Linux x86_64.

### Scaling with concurrency

| Concurrency | Throughput     | p50     | p99     |
|-------------|----------------|---------|---------|
| 5           | 17,315 req/s   | 0.25 ms | 0.57 ms |
| 10          | 20,909 req/s   | 0.39 ms | 1.10 ms |
| 25          | 29,071 req/s   | 0.70 ms | 1.64 ms |
| 50          | 32,000 req/s   | 1.25 ms | 2.90 ms |
| 100         | 32,502 req/s   | 2.42 ms | 6.03 ms |

### Compared to other tools (concurrency=100, 10 s)

| Tool     | Throughput     | p50     | p99     |
|----------|----------------|---------|---------|
| httpress | 32,502 req/s   | 2.42 ms | 6.03 ms |
| wrk¹     | 39,135 req/s   | 2.47 ms | 5.42 ms |
| hey      | 27,182 req/s   | 3.50 ms | 9.40 ms |

¹ wrk uses 4 OS threads (`-t4 -c100`); httpress and hey each run in a single process.

### Sample output

```
Target: http://localhost:3000 Get
Concurrency: 100
Stop condition: Duration(10s)

Starting benchmark with 100 workers...

--- Benchmark Complete ---
Requests:     325107 total, 325107 success, 0 errors
Duration:     10.00s
Throughput:   32502.23 req/s
Transferred:  0.62 MB

Latency:
  Min:    175us
  Max:    45.12ms
  Mean:   2.52ms
  p50:    2.42ms
  p90:    3.91ms
  p95:    4.43ms
  p99:    6.03ms

Status codes:
  200: 325107
```

## Installation

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
httpress = "0.5"
tokio = { version = "1", features = ["full"] }
```

### As a CLI Tool

```bash
cargo install httpress
```

## Quick Start

```rust
use httpress::Benchmark;
use std::time::Duration;

let results = Benchmark::builder()
    .url("http://localhost:3000")
    .concurrency(50)
    .duration(Duration::from_secs(10))
    .show_progress(true)
    .build()?
    .run()
    .await?;

results.print();
```

## Library Usage

### Basic Benchmark

```rust
let results = Benchmark::builder()
    .url("http://localhost:3000")
    .concurrency(50)
    .requests(1000)
    .show_progress(true)
    .build()?
    .run()
    .await?;
```

### Custom Request Generation

```rust
.request_fn(|ctx: RequestContext| {
    let user_id = ctx.request_number % 100;
    RequestConfig {
        url: format!("http://localhost:3000/user/{}", user_id),
        method: HttpMethod::Get,
        headers: HashMap::new(),
        body: None,
    }
})
```

### Dynamic Rate Control

```rust
.rate_fn(|ctx: RateContext| {
    let progress = (ctx.elapsed.as_secs_f64() / 10.0).min(1.0);
    100.0 + (900.0 * progress)  // Ramp from 100 to 1000 req/s
})
```

### Hook System

```rust
.after_request(|ctx: AfterRequestContext| {
    if let Some(status) = ctx.status {
        if status >= 500 {
            return HookAction::Retry;
        }
    }
    HookAction::Continue
})
```

For complete API documentation, see [docs.rs/httpress](https://docs.rs/httpress).

## CLI Usage

### Basic Examples

```bash
# Run benchmark with 100 concurrent connections for 30 seconds
httpress http://example.com -c 100 -d 30s

# Fixed number of requests with rate limiting
httpress http://example.com -n 10000 -r 1000

# POST request with headers and body
httpress http://example.com/api -m POST \
  -H "Content-Type: application/json" \
  -b '{"key": "value"}'
```

### Options

| Flag                | Description                  | Default |
| ------------------- | ---------------------------- | ------- |
| `-n, --requests`    | Total number of requests     | -       |
| `-d, --duration`    | Test duration (e.g. 10s, 1m) | -       |
| `-c, --concurrency` | Concurrent connections       | 10      |
| `-r, --rate`        | Rate limit (req/s)           | -       |
| `-m, --method`      | HTTP method                  | GET     |
| `-H, --header`      | HTTP header (repeatable)     | -       |
| `-b, --body`        | Request body                 | -       |
| `-t, --timeout`     | Request timeout in seconds   | 30      |

### Example Output

See [Performance](#performance) for real benchmark output and concurrency-scaling numbers.

## Examples

The `examples/` directory contains:

- **basic_benchmark.rs** - Simple benchmark example
- **custom_requests.rs** - Dynamic request generation with request_fn
- **rate_ramping.rs** - Rate control with rate_fn
- **hooks_metrics.rs** - Custom metrics collection using hooks

Run examples with:

```bash
cargo run --example basic_benchmark
```
