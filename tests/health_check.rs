//! tests/health_check.rs

use reqwest::Client;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
fn spawn_app() {
    print!("开始了");
    let server = zero2prod::run().expect("这个是一个错误");
    let _ = tokio::spawn(server);
}
#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = Client::new();
    let response = client.get("http://127.0.0.1:8089/health_check")
        .send()
        .await
        .expect(" Failed to execute request");
    println!("{:?}", response);
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)