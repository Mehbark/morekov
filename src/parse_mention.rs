use regex::Regex;
use serde_json::Value;

pub fn is_mention(response: &Value) -> bool {
    if let Value::Object(obj) = response {
        obj.get("type") == Some(&Value::String("mention".to_string()))
    } else {
        false
    }
}

/// Remove any mentions (no shilling or polluting >:()
pub fn strip_mention_content(content: &str) -> String {
    let md_parsed = html2md::parse_html(content);
    let re = Regex::new("\\[@.+\\]\\(.*\\)").unwrap();

    re.replace_all(&md_parsed, "").to_string()
}
