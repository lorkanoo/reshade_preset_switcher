use crate::addon::Addon;
use crate::util::is_on_character_select;
use function_name::named;
use log::{debug, info};
use std::sync::MutexGuard;

const RETRY_COUNT: usize = 10;

#[named]
pub fn process_preset_rules(new_map_id: u32) {
    let mut rule_index_to_activate = None;
    Addon::lock().context.process_manually = false;
    if !is_on_character_select() {
        debug!(
            "[{}] Not on character select, processing rules",
            function_name!()
        );
        let addon = Addon::lock();
        debug!(
            "[{}] List of rules: {:?}",
            function_name!(),
            addon.config.preset_rules
        );
        for (rule_index, preset_rule) in addon.config.preset_rules.iter().enumerate() {
            debug!("[{}] processing rule {:?}", function_name!(), preset_rule);
            let result = preset_rule.evaluate(&addon.context, &new_map_id);
            debug!(
                "[{}] rule {:?} evaluated with result {:?}",
                function_name!(),
                preset_rule,
                result
            );
            if let Ok(should_activate) = result.activate_rule {
                if should_activate {
                    rule_index_to_activate = Some(rule_index);
                    break;
                }
            }
        }
    } else {
        debug!(
            "[{}] On character select, not processing rules",
            function_name!()
        );
    }
    activate_preset_rule(Addon::lock(), rule_index_to_activate);
}

#[named]
pub fn activate_preset_rule(mut addon: MutexGuard<Addon>, rule_index_to_activate: Option<usize>) {
    debug!(
        "[{}] Activating preset rule with index {:?}",
        function_name!(),
        rule_index_to_activate
    );
    let rule_to_activate;
    if let Some(rule_index) = rule_index_to_activate {
        rule_to_activate = addon
            .config
            .preset_rules
            .get_mut(rule_index);
    } else {
        rule_to_activate = addon.config.preset_rules.last_mut();
        info!("[{}] Activating default preset", function_name!());
    }
    if let Some(rule) = rule_to_activate {
        let rule = rule.clone();
        addon.context.reshade.verify_activation = Some((rule.preset_path.clone(), RETRY_COUNT));
        let reshade_context = &addon.context.reshade.clone();
        //drop to unlock threads
        drop(addon);
        rule.activate(reshade_context);
    }
}
