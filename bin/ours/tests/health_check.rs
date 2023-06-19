#[tokio::test]
async fn health_check_works() {
    let address = ours::test_helpers::setup_server().await;

    let resp = reqwest::get(format!("{address}/health_check"))
        .await
        .expect("Failed to make request");
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.content_length(), Some(0));
}
