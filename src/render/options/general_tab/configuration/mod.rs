use crate::addon::Addon;
use crate::render::options::ERROR_COLOR;
use crate::render::{shorten_path, UiExtended};
use crate::thread::select_reshade_ini_file_thread;
use nexus::imgui::{TreeNodeFlags, Ui};

impl Addon {
    pub fn render_configuration(&mut self, ui: &Ui) {
        if ui.collapsing_header(
            "Configuration##rps",
            TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
        ) {
            let path_str = self.config.reshade.ini_path.display().to_string();

            let mut path = "Select ReShade.ini".to_string();
            if !path_str.is_empty() {
                path = shorten_path(path_str);
                if self.context.reshade.preset_shortcut_paths.is_empty() {
                    ui.text_colored(
                        ERROR_COLOR,
                        "Configure keybinds for the presets in ReShade:",
                    );
                    ui.text_disabled("1. Right-click a preset name in the preset list and choose a key.\n2. Switch to different preset in ReShade manually to save the changes.");
                    ui.spacing();
                }
            }

            ui.selected_file("ReShade.ini location", "##reshade_ini", &mut path, || {
                select_reshade_ini_file_thread()
            });
            ui.new_line();
        }
    }
}
