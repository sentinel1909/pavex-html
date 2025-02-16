// app/src/routes/index.rs

// dependencies
use pavex::response::{body::Html, Response};
use tera::{Context, Error, Tera};

// function which provides an error handler for the index handler
pub fn tera_error2response(e: &pavex::Error) -> Response {
    Response::internal_server_error().set_typed_body(e.to_string())
}

// handler which returns the index template
pub fn get(template: &Tera) -> Result<Response, Error> {
    let mut context = Context::new();
    context.insert("title", "Hello World");
    context.insert("message", "Hello, World!");
    let body: Html = template.render("base.html", &context)?.into();

    let response = Response::ok().set_typed_body(body);

    Ok(response)
}
