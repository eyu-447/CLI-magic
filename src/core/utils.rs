pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        first.to_uppercase().collect::<String>() + chars.as_str()
    } else {
        String::new()
    }
}
