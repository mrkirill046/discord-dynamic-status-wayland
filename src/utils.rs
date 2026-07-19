pub fn normalize_class(class: &str) -> String {
    class.rsplit('.').next().unwrap_or(class).to_string()
}
