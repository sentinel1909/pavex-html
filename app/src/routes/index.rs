// app/src/routes/index.rs

// dependencies
use crate::template::markdown_to_html;
use pavex::cookie::{ResponseCookie, ResponseCookies};
use pavex::response::{body::Html, Response};
use tera::{Context, Error, Tera};

// function which provides an error handler for the index handler
pub fn tera_error2response(e: &pavex::Error) -> Response {
    Response::internal_server_error().set_typed_body(e.to_string())
}

// handler which returns the index template
pub fn get(response_cookies: &mut ResponseCookies, template: &Tera) -> Result<Response, Error> {
    let cookie = ResponseCookie::new("message", "I\'m a cookie!");
    response_cookies.insert(cookie);

    let message = match response_cookies.get("message") {
        Some(cookie) => cookie.value(),
        None => "No cookie for you!"
    };

    let index_content = include_str!("../../../content/index.md");
    let index_html = markdown_to_html(index_content);

    let mut context = Context::new();
    context.insert("title", "Hello Pavex");
    context.insert("message", message);
    context.insert("content", &index_html);
    let body: Html = template.render("base.html", &context)?.into();

    let response = Response::ok().set_typed_body(body);

    Ok(response)
}
