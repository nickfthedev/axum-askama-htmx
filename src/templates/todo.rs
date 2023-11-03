use askama::Template;

// Sample Template
#[derive(Template)]
#[template(path = "todo.html")]
pub struct TodoTemplate<'a> {
    pub name: &'a str,
}
