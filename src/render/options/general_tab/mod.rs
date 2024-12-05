mod configuration;
mod rule_edit;

use crate::addon::Addon;
use crate::config::preset_rule::{PresetRule, RuleValidationError};
use crate::context::ui::UiContext;
use crate::render::options::{ERROR_COLOR, SUCCESS_COLOR};
use nexus::imgui::{Direction, MenuItem, StyleColor, TreeNodeFlags, Ui};
use crate::render::UiAction;

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
                Self::render_move_buttons(
                    ui,
                    &mut ui_actions,
                    last_rule_index,
                    rule_index,
                    &mut default_label,
                );
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
                Self::render_more_options(
                    &mut ui_actions,
                    &mut self.context.ui,
                    rule_index,
                    ui,
                );
            }
            self.post_render_rules_actions(ui_actions);
        }
    }

    fn render_move_buttons(
        ui: &Ui,
        ui_actions: &mut Vec<UiAction>,
        last_rule_index: usize,
        rule_index: usize,
        mut default_label: &mut &str,
    ) {
        Self::render_move_up_button(ui, ui_actions, rule_index);
        Self::render_move_down_button(
            ui,
            ui_actions,
            last_rule_index,
            rule_index,
            &mut default_label,
        );
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

    fn render_move_down_button(
        ui: &Ui,
        ui_actions: &mut Vec<UiAction>,
        last_rule_index: usize,
        rule_index: usize,
        default_label: &mut &str,
    ) {
        if last_rule_index == rule_index {
            *default_label = "[default]";
            let color = ui.style_color(StyleColor::TextDisabled);
            let style = ui.push_style_color(StyleColor::Text, color);
            ui.arrow_button("", Direction::Down);
            style.end();
        } else if ui.arrow_button(format!("##pr_down{}", rule_index), Direction::Down) {
            ui_actions.push(UiAction::MoveDown(rule_index));
        }
    }

    fn render_move_up_button(ui: &Ui, ui_actions: &mut Vec<UiAction>, rule_index: usize) {
        if rule_index != 0 {
            if ui.arrow_button(format!("##pr_up{}", rule_index), Direction::Up) {
                ui_actions.push(UiAction::MoveUp(rule_index));
            }
            ui.same_line();
        } else {
            let color = ui.style_color(StyleColor::TextDisabled);
            let style = ui.push_style_color(StyleColor::Text, color);
            ui.arrow_button("", Direction::Up);
            style.end();
            ui.same_line();
        }
    }

    fn post_render_rules_actions(
        &mut self,
        ui_actions: Vec<UiAction>,
    ) {
        for action in ui_actions {
            match action {
                UiAction::MoveDown(i) => self.config.preset_rules.swap(i, i + 1),
                UiAction::MoveUp(i) => self.config.preset_rules.swap(i, i - 1),
                UiAction::Delete(i) => {
                    self.config.preset_rules.remove(i);
                    ()
                },
                UiAction::Clone(i) => {
                    if let Some(rule) = self.config.preset_rules.get(i) {
                        let mut new_rule = rule.clone();
                        new_rule.rule_name = format!("{} (1)", new_rule.rule_name);
                        self.config.preset_rules.insert(0, new_rule);
                    }
                }
            }
        }
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
