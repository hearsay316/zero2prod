//! tests/health_check.rs

use std::net::TcpListener;
use reqwest::Client;


// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
fn spawn_app() -> String {
    print!("开始了");
    let listener = TcpListener::bind("127.0.0.1:0").expect("一个错误:port");
    println!("{:?}", listener);
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("这个是一个错误");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = Client::new();
    let response = client.get(&format!("{}/health_check", address))
        .send()
        .await
        .expect(" Failed to execute request");
    println!("{:?}", response);
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    // art
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client.post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());
}
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![("name=le%20guin", "missing the email"),
                          ("email=ursula_le_guin%40gmail.com", "missing the name"),
                          ("", "missing both name email")
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client.post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400, response.status().as_u16(),
                   "The API did not fail with 400 Bad Request when the payload was {}.",
                   error_message
        );
    }
}