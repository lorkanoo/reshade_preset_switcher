pub mod ui;

pub fn shorten_path(path_str: String) -> String {
    let parts: Vec<&str> = path_str.split(r#"\"#).collect();
    let last_three: Vec<&str> = parts
        .iter()
        .rev()
        .take(3)
        .copied()
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect();
    format!("..\\{}", last_three.join("\\"))
}
