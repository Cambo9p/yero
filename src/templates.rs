use askama::Template;


#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}


#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
    name: String,
}
