use crate::addon::Addon;
use crate::config::preset_rule::{PresetRule, RuleValidationError};
use crate::render::options::{ERROR_COLOR, SUCCESS_COLOR};
use crate::render::UiExtended;
use crate::thread::select_reshade_ini_file_thread;
use nexus::imgui::{Direction, Ui};
use std::collections::HashMap;

impl Addon {
    pub fn render_general_tab(&mut self, ui: &Ui) {
        ui.spacing();
        if let Some(rule_under_edit_index) = &self.context.ui.rule_under_edit_index {
            self.render_rule_edit(*rule_under_edit_index, ui);
        } else {
            if self.config.valid() && self.context.valid() {
                self.render_rules(ui);
            }
            self.render_configuration(ui);
        }
    }

    fn render_rules(&mut self, ui: &Ui) {
        ui.header("Rules");
        if self.config.preset_rules.is_empty() {
            ui.text_disabled("No rules defined");
            ui.spacing();
        } else {
            let mut move_down_index = None;
            let mut move_up_index = None;
            let mut delete_index = None;
            let mut active_labeled = false;
            if let Some(_t) = ui.begin_table("rules", 4) {
                ui.table_next_row();
                let last_rule_index = self.config.preset_rules.len() - 1;
                for (rule_index, rule) in self.config.preset_rules.iter().enumerate() {
                    let mut default_label = "";
                    ui.table_next_column();
                    if rule_index != 0 {
                        if ui.arrow_button(format!("##pr_up{}", rule_index), Direction::Up) {
                            move_up_index = Some(rule_index);
                        }
                        ui.same_line();
                    }
                    if last_rule_index == rule_index {
                        default_label = "[default]";
                    } else if ui.arrow_button(format!("##pr_down{}", rule_index), Direction::Down) {
                        move_down_index = Some(rule_index);
                    }
                    ui.table_next_column();
                    ui.text(&rule.rule_name);
                    ui.table_next_column();
                    if let Err(RuleValidationError::NoPresetSelected) = rule.validate() {
                        ui.text_colored(ERROR_COLOR, "[invalid preset]");
                        ui.same_line();
                    }

                    if self.context.reshade.active_preset_path == rule.preset_path
                        && !active_labeled
                    {
                        active_labeled = true;
                        ui.text_colored(SUCCESS_COLOR, "[active]");
                        ui.same_line();
                    }
                    if !default_label.is_empty() {
                        ui.text(default_label);
                    }

                    ui.table_next_column();
                    if ui.button(format!("Edit##{}", rule_index)) {
                        self.context.ui.rule_under_edit_index = Some(rule_index);
                    }
                    ui.same_line();
                    if ui.button(format!("Delete##{}", rule_index)) {
                        delete_index = Some(rule_index);
                    }
                }
                if let Some(rule_index) = move_down_index {
                    self.config.preset_rules.swap(rule_index, rule_index + 1);
                }
                if let Some(rule_index) = move_up_index {
                    self.config.preset_rules.swap(rule_index, rule_index - 1);
                }
                if let Some(rule_index) = delete_index {
                    self.config.preset_rules.remove(rule_index);
                }
            }
        }
        if ui.button("New rule") {
            self.config.preset_rules.insert(0, PresetRule::default());
            self.context.ui.rule_under_edit_index = Some(0);
        }
        ui.same_line();
    }

    fn render_configuration(&mut self, ui: &Ui) {
        ui.section("Configuration");
        let path_str = self.config.reshade.ini_path.display().to_string();

        let mut path = "Select ReShade.ini".to_string();
        if !path_str.is_empty() {
            path = Self::shorten_path(path_str);
            if self
                .context
                .reshade
                .previous_preset_key_combination
                .is_none()
            {
                ui.text_colored(
                    ERROR_COLOR,
                    "Configure missing keybind in reshade: Settings -> Previous preset key",
                )
            }
            if self.context.reshade.next_preset_key_combination.is_none() {
                ui.text_colored(
                    ERROR_COLOR,
                    "Configure missing keybind in reshade: Settings -> Next preset key",
                )
            }
        }

        ui.selected_file("ReShade.ini location", "##reshade_ini", &mut path, || {
            select_reshade_ini_file_thread()
        });
        ui.spacing();
    }

    fn shorten_path(path_str: String) -> String {
        let parts: Vec<&str> = path_str.split(r#"\"#).collect();
        let last_three: Vec<&str> = parts
            .iter()
            .rev()
            .take(3)
            .copied()
            .collect::<Vec<&str>>()
            .into_iter()
            .rev()
            .collect();
        format!("..\\{}", last_three.join("\\"))
    }

    fn render_rule_edit(&mut self, rule_index: usize, ui: &Ui) {
        ui.spacing();
        if ui.button("Save and close") {
            self.context.ui.rule_under_edit_index = None;
            self.context.ui.map_search_term = "".to_string();
        }
        ui.same_line();
        if ui.button("Delete") {
            self.config.preset_rules.remove(rule_index);
            self.context.ui.rule_under_edit_index = None;
        }
        ui.spacing();
        let rule = self.config.preset_rules.get_mut(rule_index).unwrap();
        ui.header("Edit rule");
        ui.input_text("Rule name", &mut rule.rule_name).build();
        ui.new_line();
        ui.header("Activation conditions");
        ui.header("When on maps:");
        let map_names = &self.context.ui.map_names;
        render_rule_maps(map_names, rule, ui, &mut self.context.ui.map_search_term);
        ui.section("Preset to activate");
        ui.input_text(
            "Preset to activate##preset_loc",
            &mut rule
                .preset_path
                .file_stem()
                .unwrap_or("Select preset from options below".as_ref())
                .to_str()
                .unwrap()
                .to_string(),
        )
        .hint("Preset")
        .auto_select_all(true)
        .read_only(true)
        .build();
        if !rule.preset_path.exists() {
            ui.text_colored(ERROR_COLOR, "Invalid preset selected");
        }
        for chunk in self.context.reshade.presets.chunks(4) {
            for preset_path in chunk {
                if rule.preset_path == *preset_path {
                    continue;
                }
                if ui.button(preset_path.file_stem().unwrap().to_str().unwrap()) {
                    rule.preset_path = preset_path.clone();
                }
                ui.same_line();
            }
            ui.new_line();
        }
        ui.section("Additional information:");
        if let Some(m) = self.context.mumble {
            ui.text(format!("Current map id: {}", m.read_map_id()));
        }
        ui.spacing();
    }
}

fn render_rule_maps(
    map_names: &HashMap<String, String>,
    preset_rule: &mut PresetRule,
    ui: &Ui,
    map_search_term: &mut String,
) {
    if preset_rule.maps.is_empty() {
        ui.text_disabled("No maps");
    } else {
        let mut to_remove = Vec::new();
        if let Some(_t) = ui.begin_table("rule_maps", 3) {
            ui.table_next_row();
            for (i, map_id) in preset_rule.maps.iter().enumerate() {
                ui.table_next_column();
                ui.text_colored(ERROR_COLOR, "[X]");
                ui.same_line_with_pos(-10f32);
                if ui.invisible_button(format!("-##prm{}", map_id), [30f32, 30f32]) {
                    to_remove.push(i);
                }
                ui.same_line_with_pos(24f32);
                let map_id_str = &map_id.to_string();
                let map_name = map_names.get(map_id_str).unwrap_or(map_id_str);
                ui.text(map_name);
            }
        }
        for map_index in to_remove {
            preset_rule.maps.remove(map_index);
        }
    }

    ui.spacing();
    ui.input_text("Search maps", map_search_term).build();
    let search_term = &map_search_term.to_lowercase();
    search_maps(ui, search_term, map_names, preset_rule);
}

fn search_maps(
    ui: &Ui,
    search_term: &String,
    map_names: &HashMap<String, String>,
    preset_rule: &mut PresetRule,
) {
    if !search_term.is_empty() {
        let search_results: Vec<(&String, &String)> = map_names
            .iter()
            .filter(|(map_id, map_name)| {
                let map_id_u32 = &map_id.parse().unwrap();
                format!("{} ({})", map_name.to_lowercase(), map_id).contains(search_term)
                    && !preset_rule.maps.contains(map_id_u32)
            })
            .take(6)
            .collect();
        for chunk in search_results.chunks(2) {
            for (id, map_name) in chunk {
                if ui.button(format!("{} ({})", map_name, id)) {
                    preset_rule.maps.push(id.parse().unwrap());
                }
                ui.same_line();
            }
            ui.new_line();
        }
    }
}
