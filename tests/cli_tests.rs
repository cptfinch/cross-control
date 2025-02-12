use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_default_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-barrier")?;
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("USAGE"))
        .stderr(predicate::str::is_empty());
    
    Ok(())
}

#[test]
fn test_server_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-barrier")?;
    
    cmd.args(["--server", "--ip", "192.168.1.10", "--port", "9000"])
        .assert()
        .success();
    
    Ok(())
}

#[test]
fn test_client_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-barrier")?;
    
    cmd.args(["--ip", "192.168.1.20", "--display", "2"])
        .assert()
        .success();
    
    Ok(())
}

#[tokio::test]
async fn test_server_bind() {
    use tokio::net::TcpListener;
    
    let listener = TcpListener::bind("127.0.0.1:9090").await.unwrap();
    assert!(listener.local_addr().is_ok());
}

#[tokio::test]
async fn test_client_connect() {
    use tokio::net::TcpStream;
    
    let result = TcpStream::connect("127.0.0.1:9091").await;
    assert!(result.is_err()); // Expected to fail since no server
} 