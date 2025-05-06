use actix_web::HttpResponse;
use actix_web::http::header::ContentType;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Change Password</title>
</head>
<body>
    {msg_html}
    <form action="/admin/newsletters" method="post">
        <label>Subject
            <input
                    type="text"
                    placeholder="Enter Subject"
                    name="subject"
            />
        </label>
        <br>
        <label>HTML Content
            <br>
            <textarea
                    placeholder="Enter HTML content"
                    name="html_content"
                    cols=60
                    rows=20
            ></textarea>
        </label>
        <br>
        <label>Text Content
            <br>
            <textarea
                    placeholder="Enter Text content"
                    name="text_content"
                    cols=60
                    rows=20
            ></textarea>
        </label>
        <br>
        <button type="submit">Publish</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
        )))
}
