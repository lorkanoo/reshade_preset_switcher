mod preset_rule;

use crate::addon::Addon;
use crate::config::game_dir;
use crate::thread::preset_rule::process_preset_rules;
use crate::util::game_has_focus;
use crate::util::reshade::load_reshade_context;
use function_name::named;
use log::{debug, info};
use rfd::FileDialog;
use std::thread;
use std::time::Duration;

#[named]
pub fn background_thread() {
    Addon::threads().push(thread::spawn(|| loop {
        if !Addon::lock().context.run_background_thread {
            break;
        }
        clean_finished_threads();

        if !Addon::lock().config.valid() {
            info!(
                "[{}] Reshade config not valid, skipping background processing",
                function_name!()
            );
        } else {
            let reshade_ini_path = Addon::lock().config.reshade.ini_path.clone();
            load_reshade_context(&reshade_ini_path);
            if !Addon::lock().context.valid() {
                info!(
                    "[{}] Reshade context not valid, skipping background processing",
                    function_name!()
                );
            } else if game_has_focus() {
                let mut new_map_id: u32 = 0;
                if Addon::lock().context.map_changed(&mut new_map_id)
                    || Addon::lock().context.time_period_changed(&mut new_map_id)
                {
                    process_preset_rules(new_map_id);
                }
            }
        }

        thread::sleep(Duration::from_millis(500));
    }));
}

pub fn select_reshade_ini_file_thread() {
    Addon::threads().push(thread::spawn(move || {
        if let Some(file) = FileDialog::new()
            .set_title("Select ReShade.ini location")
            .set_directory(game_dir())
            .add_filter("Configuration file (.ini)", &["ini"])
            .pick_file()
        {
            load_reshade_context(&file);
        }
    }));
}

#[named]
fn clean_finished_threads() {
    Addon::threads().retain(|handle| {
        if handle.is_finished() {
            debug!("[{}] removed finished thread", function_name!());
            false
        } else {
            debug!("[{}] thread in progress..", function_name!());
            true
        }
    });
}
