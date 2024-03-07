use askama::Template;


#[derive(Template)]
#[template(path = "home.html")]
pub struct Home {}


#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
    name: String,
}


#[derive(Template)]
#[template(path = "register.html")]
pub struct Register {}
