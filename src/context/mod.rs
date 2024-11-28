pub mod reshade_context;
mod ui;

use crate::addon::Addon;
use crate::context::reshade_context::ReshadeContext;
use crate::context::ui::UiContext;
use nexus::data_link::get_mumble_link;
use nexus::data_link::mumble::MumblePtr;
use std::sync::MutexGuard;

#[derive(Debug, Clone)]
pub struct Context {
    pub mumble: Option<MumblePtr>,
    pub run_background_thread: bool,
    pub previous_map_id: u32,
    pub ui: UiContext,
    pub reshade: ReshadeContext,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            mumble: get_mumble_link(),
            run_background_thread: true,
            previous_map_id: 0,
            ui: UiContext::default(),
            reshade: ReshadeContext::default(),
        }
    }
}
impl Context {
    pub fn map_changed(&mut self, new_map_id: &mut u32) -> bool {
        if let Some(m) = self.mumble {
            let current_map = m.read_map_id();
            let result = current_map != self.previous_map_id;
            self.previous_map_id = current_map;
            *new_map_id = current_map;
            return result;
        }
        false
    }
    pub fn valid(&self) -> bool {
        self.reshade.valid()
    }
}

pub fn init_context(_addon: &mut MutexGuard<Addon>) {}
