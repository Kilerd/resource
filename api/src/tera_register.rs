use serde_json::Value;
use std::collections::HashMap;
use pulldown_cmark::Options;

pub fn markdown(content: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    match content {
        Value::String(raw_content) => {
            let mut html_output = String::new();
            let options = Options::all();
            let parser = pulldown_cmark::Parser::new_ext(&raw_content, options);
            pulldown_cmark::html::push_html(&mut html_output, parser);
            Ok(Value::String(html_output))
        },
        _ => {
            Err(tera::Error::msg("markdown filter do not support this type value"))
        }
    }
}