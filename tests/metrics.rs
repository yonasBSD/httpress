mod common;

use common::TestServer;
use httpress::{Benchmark, HookAction};

#[tokio::test]
async fn test_bytes_nonzero_for_successful_requests() {
    let server = TestServer::start().await;

    let results = Benchmark::builder()
        .url(&format!("{}/ok", server.base_url))
        .requests(100)
        .concurrency(4)
        .build()
        .unwrap()
        .run()
        .await
        .unwrap();

    assert_eq!(results.successful_requests, 100);
    // "/ok" returns "OK" = 2 bytes
    assert!(
        results.total_bytes >= 100 * 2,
        "expected at least {} bytes, got {}",
        100 * 2,
        results.total_bytes
    );
}

#[tokio::test]
async fn test_bytes_zero_for_failed_requests() {
    // Use a port that is not listening
    let results = Benchmark::builder()
        .url("http://127.0.0.1:1")
        .requests(5)
        .concurrency(1)
        .build()
        .unwrap()
        .run()
        .await
        .unwrap();

    assert_eq!(results.successful_requests, 0);
    assert_eq!(results.total_bytes, 0);
}

#[tokio::test]
async fn test_bytes_scale_with_request_count() {
    let server = TestServer::start().await;

    let results = Benchmark::builder()
        .url(&format!("{}/body/1000", server.base_url))
        .requests(10)
        .concurrency(1)
        .build()
        .unwrap()
        .run()
        .await
        .unwrap();

    assert_eq!(results.successful_requests, 10);
    assert_eq!(
        results.total_bytes,
        10 * 1000,
        "expected exactly {} bytes, got {}",
        10 * 1000,
        results.total_bytes
    );
}

#[tokio::test]
async fn test_bytes_zero_for_aborted_hooks() {
    let server = TestServer::start().await;

    let results = Benchmark::builder()
        .url(&format!("{}/ok", server.base_url))
        .requests(10)
        .concurrency(2)
        .before_request(|_ctx| HookAction::Abort)
        .build()
        .unwrap()
        .run()
        .await
        .unwrap();

    assert_eq!(results.successful_requests, 0);
    assert_eq!(results.total_bytes, 0);
}
