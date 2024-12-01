pub mod reshade_context;
pub mod time_period;
pub mod ui;

use crate::addon::Addon;
use crate::context::reshade_context::ReshadeContext;
use crate::context::time_period::{
    canthan_time_thresholds, current_time_period, tyrian_time_thresholds, CurrentTimePeriod,
};
use crate::context::ui::UiContext;
use crate::util::reshade::load_reshade_context;
use function_name::named;
use log::info;
use nexus::data_link::get_mumble_link;
use nexus::data_link::mumble::MumblePtr;

#[derive(Debug, Clone)]
pub struct Context {
    pub mumble: Option<MumblePtr>,
    pub run_background_thread: bool,
    pub previous_map_id: u32,
    pub ui: UiContext,
    pub reshade: ReshadeContext,
    pub current_time_period: CurrentTimePeriod,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            mumble: get_mumble_link(),
            run_background_thread: true,
            previous_map_id: 0,
            ui: UiContext::default(),
            reshade: ReshadeContext::default(),
            current_time_period: current_time_period(tyrian_time_thresholds()),
        }
    }
}
impl Context {
    #[named]
    pub fn map_changed(&mut self, new_map_id: &mut u32) -> bool {
        if let Some(m) = self.mumble {
            let current_map = m.read_map_id();
            let result = current_map != self.previous_map_id;
            self.previous_map_id = current_map;
            *new_map_id = current_map;

            if result {
                info!("[{}] Map changed to {}", function_name!(), new_map_id);
                if canthan_maps().contains(new_map_id) {
                    self.current_time_period = current_time_period(canthan_time_thresholds());
                } else {
                    self.current_time_period = current_time_period(tyrian_time_thresholds());
                }
                info!(
                    "[{}] Current time period updated on map change: {:?}",
                    function_name!(),
                    self.current_time_period
                );
            }
            return result;
        }
        false
    }

    pub fn valid(&self) -> bool {
        self.reshade.valid()
    }
}

pub fn init_context() {
    if Addon::lock().config.valid() {
        let reshade_ini_path = Addon::lock().config.reshade.ini_path.clone();
        load_reshade_context(&reshade_ini_path);
    }
}

fn canthan_maps() -> Vec<u32> {
    vec![1442, 1438, 1452, 1422, 1490, 1428, 1465]
}
