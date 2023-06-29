use std::collections::HashMap;

use assay::{assay, assert_eq};
use test_case::test_case;

#[assay]
async fn health_check_works() {
    let address: String = tests::setup_server().await;

    let resp = reqwest::get(format!("{address}/health_check"))
        .await
        .expect("Failed to make request");
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.content_length(), Some(0));
}

#[assay]
async fn register_returns_200_for_a_valid_form_data() {
    let address = tests::setup_server().await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{address}/register"))
        .form(
            &vec![
                ("username", "test"),
                ("password", "test"),
                ("email", "test"),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        )
        .send()
        .await
        .expect("Failed to make request");
    assert_eq!(resp.status(), 200);
}

#[test_case("", "", "" ; "missing both username and password and email")]
#[test_case("", "test", "test" ; "missing username")]
#[test_case("test", "", "test" ; "missing password")]
#[test_case("test", "test", "" ; "missing email")]
#[test_case("test", "", "" ; "missing both password and email")]
#[test_case("", "test", "" ; "missing both username and email")]
#[test_case("", "", "test" ; "missing both username and password")]
#[assay]
async fn register_returns_400_when_data_is_missing(username: &str, password: &str, email: &str) {
    let address = tests::setup_server().await;

    let mut body = HashMap::new();
    let mut if_not_empty_add = |k: &str, t: &str| {
        if !t.is_empty() {
            body.insert(k.to_string(), t.to_string());
        }
    };

    if_not_empty_add("username", username);
    if_not_empty_add("password", password);
    if_not_empty_add("email", email);

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{address}/register"))
        .form(&body)
        .send()
        .await
        .expect("Failed to make request");
    assert_eq!(resp.status(), 422);
}
