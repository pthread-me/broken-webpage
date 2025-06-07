use std::{ascii::AsciiExt, str};

#[derive(Debug)]
pub struct PageInfo{
  pub name: String,
  pub link: String,
}

#[derive(Debug)]
pub struct NavBar{
  pub breadcrumb: Vec<PageInfo>,
  pub bar:        Vec<PageInfo>,
}

// IDK if this is how i want to keep it, but for now these two 
// lists are the set of paths that i will have links to.
// ALL LINKS NEED TO BE COMPOSED OF ELEMENTS FROM THESE LISTS ONLY :)
static NAV_PATHS: [&str; 4] = ["home", "projects", "notes", "Rants"];

fn to_link<'a>(elem: &'a str) -> String{
  let mut link = String::from("/");
  link.push_str(elem);
  link

}


fn to_name(elem: &str) -> String{
  dbg!(elem);
  let mut iter = elem.chars();
  let mut name: String = iter.next().unwrap_or('?').to_ascii_uppercase().to_string();
  name.push_str(iter.as_str());
  name
}


impl NavBar{
  pub fn new(current_path: &str) -> NavBar{
    let uri: Vec<&str> = current_path.split("/").collect();
    
    let bar: Vec<PageInfo> = NAV_PATHS.iter()
            .filter(|e| {!uri.contains(e)})
            .map(|e| {PageInfo{name: to_name(e), link: to_link(e)}})
            .collect();


    let mut curr:String = String::new();
    let breadcrumb: Vec<PageInfo> = uri.iter()
          .filter(|e| {e.is_ascii() && !e.is_empty()})
          .map(move |e| {
              curr.push_str(&to_link(e));
              PageInfo{name: to_name(e), link: curr.to_owned()}
            })
          .collect();
    NavBar { breadcrumb, bar }
  }
}



