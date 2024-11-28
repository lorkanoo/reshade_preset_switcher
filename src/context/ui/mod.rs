use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct UiContext {
    pub errors: Errors,
    pub map_names: HashMap<String, String>,
    pub rule_under_edit_index: Option<usize>,
    pub map_search_term: String,
}

#[derive(Clone, Debug)]
pub struct Errors {}

impl UiContext {
    pub fn default() -> Self {
        Self {
            errors: Errors::default(),
            map_names: HashMap::new(),
            rule_under_edit_index: None,
            map_search_term: "".to_string(),
        }
    }
}

impl Errors {
    pub fn default() -> Self {
        Self {}
    }
}
