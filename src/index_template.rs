use askama::Template;

pub struct Page<'a>{
  pub name: &'a str,
  pub link: &'a str,
}

#[derive(Template)]
#[template(path="index.html")]
pub struct MainNavBar<'a>{
  pub pages: Vec<Page<'a>>,
}

