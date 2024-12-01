mod configuration;
mod rule_edit;

use crate::addon::Addon;
use crate::config::preset_rule::{PresetRule, RuleValidationError};
use crate::context::ui::UiContext;
use crate::render::options::{ERROR_COLOR, SUCCESS_COLOR};
use crate::render::UiExtended;
use nexus::imgui::{Direction, Ui};
use std::path::PathBuf;

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
                    ui.table_next_column();
                    let mut default_label = "";
                    Self::render_move_up_button(ui, &mut move_up_index, rule_index);
                    Self::render_move_down_button(
                        ui,
                        &mut move_down_index,
                        last_rule_index,
                        rule_index,
                        &mut default_label,
                    );
                    ui.table_next_column();
                    ui.text(&rule.rule_name);
                    ui.table_next_column();
                    if let Err(RuleValidationError::NoPresetSelected) = rule.validate() {
                        ui.text_colored(ERROR_COLOR, "[invalid preset]");
                        ui.same_line();
                    } else if !self
                        .context
                        .reshade
                        .preset_shortcut_paths
                        .contains(&rule.preset_path)
                    {
                        ui.text_colored(ERROR_COLOR, "[keybind missing]");
                        ui.same_line();
                    }
                    Self::render_active_label(
                        &self.context.reshade.active_preset_path,
                        ui,
                        &mut active_labeled,
                        rule,
                    );
                    if !default_label.is_empty() {
                        ui.text(default_label);
                    }

                    ui.table_next_column();
                    Self::render_rule_button_ribbon(
                        &mut self.context.ui,
                        &mut delete_index,
                        rule_index,
                        ui,
                    );
                }
                self.post_render_rules_actions(move_down_index, move_up_index, delete_index);
            }
        }
        if ui.button("New rule") {
            self.config.preset_rules.insert(0, PresetRule::default());
            self.context.ui.rule_under_edit_index = Some(0);
        }
    }

    fn render_rule_button_ribbon(
        ui_context: &mut UiContext,
        delete_index: &mut Option<usize>,
        rule_index: usize,
        ui: &Ui,
    ) {
        if ui.button(format!("Edit##{}", rule_index)) {
            ui_context.rule_under_edit_index = Some(rule_index);
        }
        ui.same_line();
        if ui.button(format!("Delete##{}", rule_index)) {
            *delete_index = Some(rule_index);
        }
    }

    fn render_active_label(
        active_preset_path: &PathBuf,
        ui: &Ui,
        active_labeled: &mut bool,
        rule: &PresetRule,
    ) {
        if *active_preset_path == rule.preset_path && !*active_labeled {
            *active_labeled = true;
            ui.text_colored(SUCCESS_COLOR, "[active]");
            ui.same_line();
        }
    }

    fn render_move_down_button(
        ui: &Ui,
        move_down_index: &mut Option<usize>,
        last_rule_index: usize,
        rule_index: usize,
        default_label: &mut &str,
    ) {
        if last_rule_index == rule_index {
            *default_label = "[default]";
        } else if ui.arrow_button(format!("##pr_down{}", rule_index), Direction::Down) {
            *move_down_index = Some(rule_index);
        }
    }

    fn render_move_up_button(ui: &Ui, move_up_index: &mut Option<usize>, rule_index: usize) {
        if rule_index != 0 {
            if ui.arrow_button(format!("##pr_up{}", rule_index), Direction::Up) {
                *move_up_index = Some(rule_index);
            }
            ui.same_line();
        }
    }

    fn post_render_rules_actions(
        &mut self,
        move_down_index: Option<usize>,
        move_up_index: Option<usize>,
        delete_index: Option<usize>,
    ) {
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
