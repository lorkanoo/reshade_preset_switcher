pub mod reshade;

use crate::addon::Addon;
use function_name::named;
use log::{debug, error};
use nexus::data_link::mumble::UiState;
use rdev::{simulate, EventType};

pub fn game_has_focus() -> bool {
    if let Some(m) = Addon::lock().context.mumble {
        return matches!(m.read_ui_state(), UiState::GAME_HAS_FOCUS);
    }
    false
}

pub fn true_if_1() -> fn(&String) -> bool {
    |value| value == "1"
}

#[named]
pub fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => debug!("[{}] Keypress sent", function_name!()),
        Err(_) => {
            error!("Could not send {:?}", event_type);
        }
    }
}
