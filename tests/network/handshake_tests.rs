#[tokio::test]
async fn test_secure_handshake() {
    let (mut client, mut server) = establish_test_connection().await;
    
    // Perform mutual authentication
    let client_result = client.authenticate().await;
    let server_result = server.authenticate().await;
    
    assert!(client_result.is_ok());
    assert!(server_result.is_ok());
    assert_eq!(client.peer_identity(), Some(server.id()));
    assert_eq!(server.peer_identity(), Some(client.id()));
}

#[tokio::test]
async fn test_handshake_rejects_invalid_credentials() {
    let (mut client, mut server) = establish_test_connection().await;
    
    client.set_credentials(b"wrong_password");
    
    let client_result = client.authenticate().await;
    let server_result = server.authenticate().await;
    
    assert!(matches!(client_result, Err(HandshakeError::AuthenticationFailed)));
    assert!(matches!(server_result, Err(HandshakeError::AuthenticationFailed)));
} 