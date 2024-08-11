//! tests/health_check.rs

use std::net::TcpListener;
use reqwest::Client;


// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
fn spawn_app()->String {
    print!("开始了");
    let listener = TcpListener::bind("127.0.0.1:0").expect("一个错误:port");
    println!("{:?}",listener);
    let  port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("这个是一个错误");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}",port)
}
#[tokio::test]
async fn health_check_works() {
   let address =  spawn_app();
    let client = Client::new();
    let response = client.get(&format!("{}/health_check",address))
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