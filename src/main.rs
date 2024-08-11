use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8089").expect("一个错误:port");

    run(listener)?.await
}