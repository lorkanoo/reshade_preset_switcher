pub mod reshade_context;
pub mod ui;

use crate::addon::Addon;
use crate::context::reshade_context::ReshadeContext;
use crate::context::ui::UiContext;
use chrono::{DateTime, Duration, Timelike, Utc};
use function_name::named;
use log::info;
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
    pub current_time_period: CurrentTimePeriod,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CurrentTimePeriod {
    Day,
    Dusk,
    Night,
    Dawn,
}

#[derive(Debug, Clone)]
pub enum TimePeriodType {
    Tyrian,
    Canthan,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            mumble: get_mumble_link(),
            run_background_thread: true,
            previous_map_id: 0,
            ui: UiContext::default(),
            reshade: ReshadeContext::default(),
            current_time_period: current_time_period(TimePeriodType::Tyrian),
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
                    self.current_time_period = current_time_period(TimePeriodType::Canthan);
                } else {
                    self.current_time_period = current_time_period(TimePeriodType::Tyrian);
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

    #[named]
    pub fn time_period_changed(&mut self, current_map_id: &mut u32) -> bool {
        let mut time_period_type = TimePeriodType::Tyrian;
        if canthan_maps().contains(current_map_id) {
            time_period_type = TimePeriodType::Canthan;
        }
        let new_period = current_time_period(time_period_type);
        if new_period != self.current_time_period {
            self.current_time_period = new_period;
            info!(
                "[{}] Time period changed to {:?}",
                function_name!(),
                self.current_time_period
            );
            return true;
        }
        false
    }

    pub fn valid(&self) -> bool {
        self.reshade.valid()
    }
}

pub fn init_context(_addon: &mut MutexGuard<Addon>) {}

fn last_even_hour(now: DateTime<Utc>) -> u32 {
    if now.hour() % 2 == 1 {
        now.hour() - 1
    } else {
        now.hour()
    }
}

fn canthan_maps() -> Vec<u32> {
    vec![1442, 1438, 1452, 1422, 1490, 1428, 1465]
}

pub fn current_time_period(time_period_type: TimePeriodType) -> CurrentTimePeriod {
    let current_time = Utc::now() + Duration::hours(1);
    let hour = last_even_hour(current_time);
    let day_start_time = current_time
        .with_hour(hour)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();
    let duration = current_time.signed_duration_since(day_start_time);
    let seconds_between = duration.num_seconds();

    match time_period_type {
        TimePeriodType::Tyrian => tyrian_time_period(&seconds_between),
        TimePeriodType::Canthan => canthan_time_period(&seconds_between),
    }
}

fn tyrian_time_period(seconds_between: &i64) -> CurrentTimePeriod {
    if seconds_between <= &2400i64 {
        return CurrentTimePeriod::Day;
    }
    if seconds_between <= &2700i64 {
        return CurrentTimePeriod::Dusk;
    }
    if seconds_between <= &5100i64 {
        return CurrentTimePeriod::Night;
    }
    if seconds_between <= &5400i64 {
        return CurrentTimePeriod::Dawn;
    }
    CurrentTimePeriod::Day
}

fn canthan_time_period(seconds_between: &i64) -> CurrentTimePeriod {
    if seconds_between <= &2100i64 {
        return CurrentTimePeriod::Day;
    }
    if seconds_between <= &2400i64 {
        return CurrentTimePeriod::Dusk;
    }
    if seconds_between <= &5700i64 {
        return CurrentTimePeriod::Night;
    }
    if seconds_between <= &6000i64 {
        return CurrentTimePeriod::Dawn;
    }
    CurrentTimePeriod::Day
}
