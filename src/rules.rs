use std::env;

use crate::manifest::Rules;

pub fn is_rule_satisfied(rule: &Rules) -> bool {
    if rule.os.is_some() {
        let os = rule.os.as_ref().unwrap();
        let target_os = &os.name;

        if target_os.is_some() {
            let target_os = target_os.as_ref().unwrap();
            let current_os = env::consts::OS;

            if current_os != target_os {
                return false;
            }
        }
    }

    if rule.features.is_some() {
        let features = rule.features.as_ref().unwrap();
        let custom_res = features.has_custom_resolution.unwrap_or(false);
        let demo = features.is_demo_user.unwrap_or(false);
        let quick_realms: bool = features.is_quick_play_realms.unwrap_or(false);

        if custom_res || demo || quick_realms {
            return false;
        }
    }

    return true;
}

pub fn is_all_rules_satisfied(rules: &Vec<Rules>) -> bool {
    for rule in rules.iter() {
        let satisfied = is_rule_satisfied(rule);
        let use_lib = rule.action == "allow";

        if satisfied && !use_lib {
            return false;
        } else if !satisfied && use_lib {
            return false;
        }
    }

    return true;
}
