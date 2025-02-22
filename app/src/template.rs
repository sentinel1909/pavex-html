// app/src/template.rs

// dependencies
use pulldown_cmark::{
    html::push_html,
    Options,
    Parser,
};
use tera::{Error, Tera};

// utility function to convert markdown content to HTML
pub fn markdown_to_html(md_content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(md_content, options);
    let mut html_output = String::new();
    push_html(&mut html_output, parser);
    html_output
}

// constructor to compile Tera templates
pub fn compile_templates() -> Result<Tera, Error> {
    let mut templates = Tera::new("templates/**/*")?;
    templates.add_template_files(vec![("templates/base.html", Some("base"))])?;
    Ok(templates)
}
