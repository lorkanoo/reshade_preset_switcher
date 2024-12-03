use crate::addon::Addon;
use crate::render::options::ERROR_COLOR;
use crate::render::{shorten_path, UiExtended};
use crate::thread::select_reshade_ini_file_thread;
use arboard::Clipboard;
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
                if self.context.ui.invalid_reshade_preset_configuration {
                    ui.text_colored(
                        ERROR_COLOR,
                        "ReShade preset keybind configuration is corrupt.\n\
                        This may happen when presets are renamed without removing keybinds.\n\
                        Presets will not switch correctly if keybinds overlap.",
                    );
                    ui.text_disabled("To fix an issue:\n\
                        1. Press 'Copy to clipboard' button to copy valid configuration.\n\
                        2. Exit the game.\n\
                        3. Open ReShade.ini and replace 'PresetShortcutKeys' and 'PresetShortcutPaths' with copied values."
                    );
                    if ui.button("Copy to clipboard") {
                        let mut clipboard = Clipboard::new().unwrap();
                        clipboard
                            .set_text(self.context.reshade.as_reshade_shortcut_configuration())
                            .unwrap();
                    }
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
