mod configuration;
mod rule_edit;

use crate::addon::Addon;
use crate::config::preset_rule::{PresetRule, RuleValidationError};
use crate::context::ui::UiContext;
use crate::render::options::{ERROR_COLOR, SUCCESS_COLOR};
use crate::render::util::ui::extended::UiExtended;
use crate::render::util::ui::{process_ui_actions_for_vec, UiAction};
use nexus::imgui::{MenuItem, TreeNodeFlags, Ui};

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
            self.render_how_to_use(ui);
        }
    }

    fn render_rules(&mut self, ui: &Ui) {
        if ui.collapsing_header(
            "Rules##rps",
            TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
        ) {
            if self.config.preset_rules.is_empty() {
                ui.text_disabled("No rules defined");
                ui.spacing();
            } else {
                self.render_rule_table(ui);
            }
            if ui.button("New rule") {
                self.config.preset_rules.insert(0, PresetRule::default());
                self.context.ui.rule_under_edit_index = Some(0);
            }
            ui.same_line();
            if ui.button("Process rules now") {
                self.context.process_manually = true;
            }
            ui.new_line();
        }
    }

    fn render_rule_table(&mut self, ui: &Ui) {
        let mut ui_actions: Vec<UiAction> = vec![];
        let mut active_labeled = false;
        if let Some(_t) = ui.begin_table("rules", 5) {
            let last_rule_index = self.config.preset_rules.len() - 1;
            for (rule_index, rule) in self.config.preset_rules.iter().enumerate() {
                ui.table_next_column();
                let mut default_label = "";
                if last_rule_index == rule_index {
                    default_label = "[default]";
                }
                ui.move_up_button(&mut ui_actions, rule_index);
                ui.move_down_button(&mut ui_actions, rule_index, last_rule_index);
                ui.table_next_column();
                ui.text(&rule.rule_name);
                ui.table_next_column();
                if let Some(key_combination) = self
                    .context
                    .reshade
                    .preset_shortcuts
                    .get_by_right(&rule.preset_path)
                {
                    ui.text_disabled(format!("{}", key_combination));
                }
                ui.table_next_column();
                if let Err(RuleValidationError::NoPresetSelected) = rule.validate() {
                    ui.text_colored(ERROR_COLOR, "[invalid preset]");
                    ui.same_line();
                } else if !self
                    .context
                    .reshade
                    .preset_shortcuts
                    .contains_right(&rule.preset_path)
                {
                    ui.text_colored(ERROR_COLOR, "[keybind missing]");
                    ui.same_line();
                }

                if self.context.reshade.active_preset_path == rule.preset_path && !active_labeled {
                    active_labeled = true;
                    ui.text_colored(SUCCESS_COLOR, "[active]");
                    ui.same_line();
                }

                if !default_label.is_empty() {
                    ui.text(default_label);
                }

                ui.table_next_column();
                Self::render_more_options(&mut ui_actions, &mut self.context.ui, rule_index, ui);
            }
            process_ui_actions_for_vec(&mut self.config.preset_rules, ui_actions);
        }
    }

    fn render_more_options(
        ui_actions: &mut Vec<UiAction>,
        ui_context: &mut UiContext,
        rule_index: usize,
        ui: &Ui,
    ) {
        if ui.button(format!("Edit##{}", rule_index)) {
            ui_context.rule_under_edit_index = Some(rule_index);
        }
        ui.same_line();
        if ui.button(format!("More..##{}", rule_index)) {
            ui.open_popup(format!("##popup{}", rule_index));
        }
        ui.popup(format!("##popup{}", rule_index), || {
            if MenuItem::new(format!("Clone##{}", rule_index)).build(ui) {
                ui_actions.push(UiAction::Clone(rule_index));
                ui.close_current_popup();
            }
            if MenuItem::new(format!("Delete##{}", rule_index)).build(ui) {
                ui_actions.push(UiAction::Delete(rule_index));
                ui.close_current_popup();
            }
        });
        ui.same_line();
    }

    fn render_how_to_use(&self, ui: &Ui) {
        if ui.collapsing_header("Usage tips##rps", TreeNodeFlags::SPAN_AVAIL_WIDTH) {
            ui.text_disabled("\
                1. Rules are processed from top to bottom.\n\
                2. First successful rule is activated.\n\
                3. If there are no matching rules, last rule on the list is used by default. Therefore conditions on a default rule may be empty.\n\
                4. On character select, default rule will be used."
            );
        }
    }
}
