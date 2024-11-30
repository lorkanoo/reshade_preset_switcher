use crate::addon::Addon;
use crate::config::preset_rule::rule_condition::rule_data::{ConditionData, TimePeriods};
use crate::config::preset_rule::rule_condition::{ConjunctionType, RuleCondition, Switch};
use crate::config::preset_rule::PresetRule;
use crate::context::reshade_context::ReshadeContext;
use crate::context::Context;
use crate::render::options::ERROR_COLOR;
use crate::render::UiExtended;
use nexus::data_link::mumble::MumblePtr;
use nexus::imgui::Ui;
use std::collections::HashMap;

impl Addon {
    pub fn render_rule_edit(&mut self, rule_index: usize, ui: &Ui) {
        self.render_button_ribbon(rule_index, ui);
        let rule = self.config.preset_rules.get_mut(rule_index).unwrap();
        ui.input_text("Rule name", &mut rule.rule_name).build();
        Self::render_activation_conditions(&mut self.context, rule, ui);
        Self::render_preset_picker(&self.context.reshade, rule, ui);
        Self::render_additional_info(&self.context.mumble, ui);
        ui.spacing();
    }

    fn render_additional_info(mumble: &Option<MumblePtr>, ui: &Ui) {
        ui.section("Additional information:");
        if let Some(m) = mumble {
            ui.text(format!("Current map id: {}", m.read_map_id()));
        }
    }

    fn render_preset_picker(reshade_context: &ReshadeContext, rule: &mut PresetRule, ui: &Ui) {
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
        for chunk in reshade_context.presets.chunks(4) {
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
    }

    fn render_activation_conditions(context: &mut Context, rule: &mut PresetRule, ui: &Ui) {
        ui.section("Activation conditions");
        if rule.conditions.is_empty() {
            ui.text_disabled("No conditions");
        }
        let mut has_map_condition = false;
        let mut has_time_condition = false;
        let mut delete_index = None;
        let mut rule_condition_iter = rule.conditions.iter_mut().enumerate().peekable();
        while let Some((condition_index, rule_condition)) = rule_condition_iter.next() {
            Self::render_condition_data(
                context,
                ui,
                &mut has_map_condition,
                &mut has_time_condition,
                rule_condition,
            );
            ui.spacing();
            if ui.button(format!("Delete##rule_condition{}", condition_index)) {
                delete_index = Some(condition_index);
            }
            if let Some((next_index, next_condition)) = rule_condition_iter.peek_mut() {
                ui.separator_disabled();
                if ui.button(format!(
                    "{}##rule_condition{}",
                    next_condition.conjunction_type, next_index
                )) {
                    next_condition.conjunction_type.switch();
                }
            }
        }
        if let Some(index) = delete_index {
            rule.conditions.remove(index);
        }
        ui.spacing();
        if !(has_map_condition && has_time_condition) {
            ui.separator_disabled();
            ui.header("Add new condition:");
        }
        if !has_map_condition {
            if ui.button("Map") {
                rule.conditions.push(RuleCondition::new(
                    ConditionData::Maps(Vec::new()),
                    ConjunctionType::And,
                ))
            }
            ui.same_line();
        }
        if !has_time_condition && ui.button("Time") {
            rule.conditions.push(RuleCondition::new(
                ConditionData::Time(TimePeriods::default()),
                ConjunctionType::And,
            ));
        }
    }

    fn render_condition_data(
        context: &mut Context,
        ui: &Ui,
        has_map_condition: &mut bool,
        has_time_condition: &mut bool,
        rule_condition: &mut RuleCondition,
    ) {
        match &mut rule_condition.data {
            ConditionData::Maps(maps) => {
                let map_names = &context.ui.map_names;
                Self::render_maps_condition_data(
                    map_names,
                    maps,
                    &mut context.ui.map_search_term,
                    ui,
                );
                *has_map_condition = true;
            }
            ConditionData::Time(time_periods) => {
                Self::render_time_condition_data(time_periods, ui);
                *has_time_condition = true;
            }
        }
    }

    fn render_time_condition_data(time_periods: &mut TimePeriods, ui: &Ui) {
        ui.header("When time is:");
        ui.checkbox("Day", &mut time_periods.day);
        ui.same_line();
        ui.checkbox("Dusk", &mut time_periods.dusk);
        ui.same_line();
        ui.checkbox("Night", &mut time_periods.night);
        ui.same_line();
        ui.checkbox("Dawn", &mut time_periods.dawn);
    }

    fn render_button_ribbon(&mut self, rule_index: usize, ui: &Ui) {
        ui.spacing();
        if ui.button("Close") {
            self.context.ui.rule_under_edit_index = None;
            self.context.ui.map_search_term = "".to_string();
        }
        ui.same_line();
        if ui.button("Delete") {
            self.config.preset_rules.remove(rule_index);
            self.context.ui.rule_under_edit_index = None;
        }
        ui.spacing();
    }

    fn render_maps_condition_data(
        map_names: &HashMap<String, String>,
        maps: &mut Vec<u32>,
        map_search_term: &mut String,
        ui: &Ui,
    ) {
        ui.header("When on maps:");
        if maps.is_empty() {
            ui.text_disabled("No maps");
        } else {
            let mut to_remove = Vec::new();
            if let Some(_t) = ui.begin_table("rule_maps", 3) {
                ui.table_next_row();
                for (i, map_id) in maps.iter().enumerate() {
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
                maps.remove(map_index);
            }
        }

        ui.spacing();
        ui.input_text("Search maps", map_search_term).build();
        let search_term = &map_search_term.to_lowercase();
        Self::search_maps(search_term, map_names, maps, ui);
    }

    fn search_maps(
        search_term: &String,
        map_names: &HashMap<String, String>,
        maps: &mut Vec<u32>,
        ui: &Ui,
    ) {
        if !search_term.is_empty() {
            let search_results: Vec<(&String, &String)> = map_names
                .iter()
                .filter(|(map_id, map_name)| {
                    let map_id_u32 = &map_id.parse().unwrap();
                    format!("{} ({})", map_name.to_lowercase(), map_id).contains(search_term)
                        && !maps.contains(map_id_u32)
                })
                .take(6)
                .collect();
            for chunk in search_results.chunks(2) {
                for (id, map_name) in chunk {
                    if ui.button(format!("{} ({})", map_name, id)) {
                        maps.push(id.parse().unwrap());
                    }
                    ui.same_line();
                }
                ui.new_line();
            }
        }
    }
}
