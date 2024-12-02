pub mod reshade;

use crate::addon::Addon;
use function_name::named;
use log::{debug, error};
use nexus::data_link::mumble::UiState;
use rdev::{simulate, EventType};

pub fn game_has_focus() -> bool {
    if let Some(m) = Addon::lock().context.links.mumble {
        return matches!(m.read_ui_state(), UiState::GAME_HAS_FOCUS);
    }
    false
}

pub fn is_in_game() -> bool {
    let mut is_gameplay = false;
    if let Some(nexus) = unsafe { Addon::lock().context.links.nexus() } {
        if nexus.is_gameplay {
            is_gameplay = true;
        }
    }
    is_gameplay
}

pub fn is_on_character_select() -> bool {
    !is_in_game()
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
