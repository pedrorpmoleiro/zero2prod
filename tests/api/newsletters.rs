use crate::helpers::{ConfirmationLinks, TestApp, assert_is_redirect_to, spawn_app};
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let _mock_guard = Mock::given(path("/v1/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(202))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscriptions(body.into())
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(&email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_links = create_unconfirmed_subscriber(app).await;

    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}

// Newsletter get form tests
#[tokio::test]
async fn you_must_be_logged_in_to_see_newsletter_form() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_publish_newsletter().await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

// Newsletter post api tests
#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;
    app.test_user.login(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0) // We assert that no request is fired (email sent)
        .mount(&app.email_server)
        .await;

    // Act Part 1 Submit newsletter form
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
    });
    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    // Act Part 2 Follow redirect
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
    // Mock verifies on Drop that we haven't sent the newsletter email
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;
    app.test_user.login(&app).await;

    Mock::given(path("/v1/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(202))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act Part 1 Submit newsletter form
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
    });
    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    // Act Part 2 Follow redirect
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
    // Mock verifies on Drop that we have sent the newsletter email
}

#[tokio::test]
async fn newsletters_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    app.test_user.login(&app).await;
    let test_cases = vec![
        (
            serde_json::json!({
                "text_content": "Newsletter body as plain text",
                "html_content": "<p>Newsletter body as HTML</p>",
            }),
            "missing title",
        ),
        (
            serde_json::json!({"title": "Newsletter!"}),
            "missing both content",
        ),
        (
            serde_json::json!({
                "title": "Newsletter!",
                "html_content": "<p>Newsletter body as HTML</p>",
            }),
            "missing text content",
        ),
        (
            serde_json::json!({
                "title": "Newsletter!",
                "text_content": "Newsletter body as plain text",
            }),
            "missing html content",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_publish_newsletter(&invalid_body).await;
        //assert_is_redirect_to(&response, "/admin/newsletters");

        // Act Part 2 Follow redirect
        // let html_page = app.get_publish_newsletter_html().await;
        // assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

// Session auth tests
#[tokio::test]
async fn you_must_be_logged_in_to_publish_newsletter_issue() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
    });
    let response = app.post_publish_newsletter(&newsletter_request_body).await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

// Not required since moved to session authentication
// #[tokio::test]
// async fn requests_missing_authorization_are_rejected() {
//     // Arrange
//     let app = spawn_app().await;
//
//     // Act
//     let response = reqwest::Client::new()
//         .post(&format!("{}/newsletters", &app.address))
//         .json(&serde_json::json!({
//             "title": "Newsletter title",
//             "content": {
//                 "text": "Newsletter body as plain text",
//                 "html": "<p>Newsletter body as HTML</p>",
//             }
//         }))
//         .send()
//         .await
//         .expect("Failed to execute request.");
//
//     // Assert
//     assert_eq!(response.status().as_u16(), 401);
//     assert_eq!(
//         r#"Basic realm="publish""#,
//         response.headers()["WWW-Authenticate"]
//     );
// }

// #[tokio::test]
// async fn non_existent_user_is_rejected() {
//     // Arrange
//     let app = spawn_app().await;
//     // Random credentials
//     let username = Uuid::new_v4().to_string();
//     let password = Uuid::new_v4().to_string();
//
//     // Act
//     let response = reqwest::Client::new()
//         .post(&format!("{}/newsletters", &app.address))
//         .basic_auth(username, Some(password))
//         .json(&serde_json::json!({
//             "title": "Newsletter title",
//             "content": {
//                 "text": "Newsletter body as plain text",
//                 "html": "<p>Newsletter body as HTML</p>",
//             }
//         }))
//         .send()
//         .await
//         .expect("Failed to execute request.");
//
//     // Assert
//     assert_eq!(response.status().as_u16(), 401);
//     assert_eq!(
//         r#"Basic realm="publish""#,
//         response.headers()["WWW-Authenticate"]
//     )
// }

// #[tokio::test]
// async fn invalid_password_is_rejected() {
//     // Arrange
//     let app = spawn_app().await;
//     let username = &app.test_user.username;
//     // Random password
//     let password = Uuid::new_v4().to_string();
//     assert_ne!(app.test_user.password, password);
//
//     let response = reqwest::Client::new()
//         .post(&format!("{}/newsletters", &app.address))
//         .basic_auth(username, Some(password))
//         .json(&serde_json::json!({
//             "title": "Newsletter title",
//             "content": {
//                 "text": "Newsletter body as plain text",
//                 "html": "<p>Newsletter body as HTML</p>",
//             }
//         }))
//         .send()
//         .await
//         .expect("Failed to execute request.");
//
//     // Assert
//     assert_eq!(401, response.status().as_u16());
//     assert_eq!(
//         r#"Basic realm="publish""#,
//         response.headers()["WWW-Authenticate"]
//     );
// }
