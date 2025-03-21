pub mod extended;

pub enum UiAction {
    MoveDown(usize),
    MoveUp(usize),
    Delete(usize),
    Clone(usize),
}

pub enum RenderResult {
    Terminated,
    Concluded,
}

pub trait UiElement {
    fn rename(&mut self, _new_name: String) {}
    fn name(&self) -> &String;
}

pub fn process_ui_actions_for_vec<T: UiElement + Clone>(
    vec: &mut Vec<T>,
    ui_actions: Vec<UiAction>,
) {
    for action in ui_actions {
        match action {
            UiAction::MoveDown(i) => vec.swap(i, i + 1),
            UiAction::MoveUp(i) => vec.swap(i, i - 1),
            UiAction::Delete(i) => {
                vec.remove(i);
            }
            UiAction::Clone(i) => {
                if let Some(t) = vec.get(i) {
                    let mut new_t = t.clone();
                    new_t.rename(format!("{} (1)", new_t.name()));
                    vec.insert(0, new_t);
                }
            }
        }
    }
}
