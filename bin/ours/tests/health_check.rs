#[tokio::test]
async fn health_check_works() {
    let _ = ours::test_helpers::setup_server().await;

    let resp = reqwest::get("http://127.0.0.1:8000/health_check")
        .await
        .expect("Failed to make request");
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.content_length(), Some(0));
}
