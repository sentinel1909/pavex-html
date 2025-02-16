// app/src/template.rs

// dependencies
use tera::{Error, Tera};

// constructor to compile Tera templates
pub fn compile_templates() -> Result<Tera, Error> {
    let mut templates = Tera::new("templates/**/*")?;
    templates.add_template_files(vec![("templates/base.html", Some("base"))])?;
    Ok(templates)
}
