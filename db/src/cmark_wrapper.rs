// A wrapper over pulldown-cmark, for convinience*
use pulldown_cmark::{html, Parser, TextMergeStream, Options};

pub fn to_html(markdown: &mut str) -> String { 
  let mut html_out = String::new();
  let parser = TextMergeStream::new(Parser::new_ext(markdown, Options::ENABLE_MATH));
  
  html::push_html(&mut html_out, parser);
  html_out
}
