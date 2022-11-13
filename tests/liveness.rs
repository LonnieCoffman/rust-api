#[tokio::test]
async fn liveness_works() {
    spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:6005/liveness")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let server = rustapi::run("127.0.0.1:6005").expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
