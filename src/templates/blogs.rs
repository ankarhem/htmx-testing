use askama::Template;

#[derive(Template)]
#[template(path = "blogs.html")]
pub struct BlogsTemplate<'a> {
    title: &'a str,
}

pub async fn handler<'a>() -> BlogsTemplate<'a> {
    BlogsTemplate { title: "blogs" }
}
