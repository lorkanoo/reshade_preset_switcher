use crate::api::gw2::fetch_map_names_thread;
use crate::config::{config_dir, migrate_configs, Config};
use crate::context::{init_context, Context};
use crate::thread::background_thread;
use function_name::named;
use log::info;
use nexus::gui::{register_render, RenderType};
use nexus::quick_access::add_quick_access_context_menu;
use std::fs;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread::JoinHandle;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
static MULTITHREADED_ADDON: MultithreadedAddon = MultithreadedAddon {
    addon: OnceLock::new(),
    threads: OnceLock::new(),
};

pub struct MultithreadedAddon {
    pub addon: OnceLock<Mutex<Addon>>,
    pub threads: OnceLock<Mutex<Vec<JoinHandle<()>>>>,
}

#[derive(Debug, Default)]
pub struct Addon {
    pub config: Config,
    pub context: Context,
}

impl Addon {
    pub fn lock() -> MutexGuard<'static, Addon> {
        MULTITHREADED_ADDON
            .addon
            .get_or_init(|| Mutex::new(Addon::default()))
            .lock()
            .unwrap()
    }

    pub fn threads() -> MutexGuard<'static, Vec<JoinHandle<()>>> {
        MULTITHREADED_ADDON
            .threads
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .unwrap()
    }

    #[named]
    pub fn load() {
        info!(
            "[{}] Loading reshade_preset_switcher v{}",
            function_name!(),
            VERSION
        );
        let _ = fs::create_dir(config_dir());
        {
            if let Some(config) = Config::try_load() {
                Addon::lock().config = config;
            }
        }

        // Addon::lock().config.preset_rules.push(PresetRule {
        //     maps: vec![50],
        //     preset_name: "Lions Arch".to_string(),
        // });

        migrate_configs(&mut Addon::lock());
        init_context(&mut Addon::lock());
        fetch_map_names_thread();
        background_thread();

        register_render(
            RenderType::OptionsRender,
            nexus::gui::render!(|ui| Addon::lock().render_options(ui)),
        )
        .revert_on_unload();

        add_quick_access_context_menu(
            "reshade_preset_switcher",
            None::<&str>,
            nexus::gui::render!(|ui| Addon::lock().render_quick_access(ui)),
        )
        .revert_on_unload();
        info!("[{}] reshade_preset_switcher loaded", function_name!());
    }
    #[named]
    pub fn unload() {
        info!(
            "[{}] Unloading reshade_preset_switcher v{VERSION}",
            function_name!()
        );
        Self::lock().context.run_background_thread = false;
        let mut threads = Self::threads();
        while let Some(thread) = threads.pop() {
            info!("[{}] Waiting for a thread to end..", function_name!());
            match thread.join() {
                Ok(_) => info!("[{}] Thread unloaded successfully", function_name!()),
                Err(_) => log::error!("[{}] Thread unloaded with error", function_name!()),
            }
        }
        let addon = &mut Self::lock();
        info!("[{}] Saving configuration..", function_name!());
        addon.config.save();
        info!("[{}] reshade_preset_switcher unloaded", function_name!());
    }
}
