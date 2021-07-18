#[actix_rt::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8080/health_check")
        .send()
        .await
        .expect("Failed to get health check");


    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> () {
    let server = skel::run().expect("failed to bind to address");

    let _ = tokio::spawn(server);
}