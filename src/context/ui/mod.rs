use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct UiContext {
    pub map_names: HashMap<String, String>,
    pub rule_under_edit_index: Option<usize>,
    pub map_search_term: String,
    pub blacklist_map_search_term: String,
    pub invalid_reshade_preset_configuration: bool,
}

#[derive(Clone, Debug)]
pub struct Errors {}

impl Default for UiContext {
    fn default() -> Self {
        Self {
            map_names: HashMap::new(),
            rule_under_edit_index: None,
            map_search_term: "".to_string(),
            blacklist_map_search_term: "".to_string(),
            invalid_reshade_preset_configuration: false,
        }
    }
}
