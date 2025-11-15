use crate::context::{canthan_time_maps, Context};
use chrono::{DateTime, Duration, Timelike, Utc};
use function_name::named;
use log::{error, info};
use nexus::rtapi::{TimeOfDay, WorldData};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CurrentTimePeriod {
    Day,
    Dusk,
    Night,
    Dawn,
}

impl From<TimeOfDay> for CurrentTimePeriod {
    fn from(time_of_day: TimeOfDay) -> Self {
        match time_of_day {
            TimeOfDay::Dawn => CurrentTimePeriod::Dawn,
            TimeOfDay::Day => CurrentTimePeriod::Day,
            TimeOfDay::Dusk => CurrentTimePeriod::Dusk,
            TimeOfDay::Night => CurrentTimePeriod::Night,
        }
    }
}

impl Context {
    #[named]
    pub fn time_period_changed(&mut self, current_map_id: &mut u32) -> bool {
        let mut time_thresholds = tyrian_time_thresholds();
        if canthan_time_maps().contains(current_map_id) {
            time_thresholds = canthan_time_thresholds();
        }
        let new_period = if let Some(rtapi) = &self.links.rtapi {
            let world_data = unsafe { WorldData::read(rtapi) };
            match world_data.time_of_day {
                Ok(time_of_day) => CurrentTimePeriod::from(time_of_day),
                Err(e) => {
                    error!("Error reading rtapi time of day: {}", e);
                    current_time_period_with_default_detection(time_thresholds)
                }
            }
        } else {
            current_time_period_with_default_detection(time_thresholds)
        };

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
}

fn last_even_hour(now: DateTime<Utc>) -> u32 {
    if now.hour() % 2 == 1 {
        now.hour() - 1
    } else {
        now.hour()
    }
}

pub fn current_time_period_with_default_detection(thresholds: (i64, i64, i64, i64)) -> CurrentTimePeriod {
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

    time_period(&seconds_between, thresholds)
}

fn time_period(seconds_between: &i64, time_thresholds: (i64, i64, i64, i64)) -> CurrentTimePeriod {
    if seconds_between <= &time_thresholds.0 {
        return CurrentTimePeriod::Day;
    }
    if seconds_between <= &time_thresholds.1 {
        return CurrentTimePeriod::Dusk;
    }
    if seconds_between <= &time_thresholds.2 {
        return CurrentTimePeriod::Night;
    }
    if seconds_between <= &time_thresholds.3 {
        return CurrentTimePeriod::Dawn;
    }
    CurrentTimePeriod::Day
}

pub fn tyrian_time_thresholds() -> (i64, i64, i64, i64) {
    (2400, 2700, 5100, 5400)
}

pub fn canthan_time_thresholds() -> (i64, i64, i64, i64) {
    (2100, 2400, 5700, 6000)
}
