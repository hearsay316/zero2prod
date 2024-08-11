//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
// #[tokio::test]
// async fn health_check_works() {
//     // spawn_app();
//     let client = reqwest::Client::new();
//     let response = client.get("http://127.0.0.1:8090/health_check")
//         .send()
//         .await
//         .expect(" Failed to execute request");
//       assert!(response.status().is_success());
//      assert_eq!(Some(0),response.content_length());
// }
#[warn(dead_code)]
 fn spawn_app(){
  let server =  zero2prod::run().expect("这个是一个错误");
    let _ = tokio::spawn(server);
}


// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
     spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8089/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
println!("{:?}",response);
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
