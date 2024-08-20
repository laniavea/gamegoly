use rand::Rng;
use slint::{Model, VecModel};

use crate::{FieldAdapter, InfoPanelAdapter, SpecialDice, Condition, ListData};

impl ListData {
    pub fn make_roll(&self) -> slint::SharedString {
        let elements_count = self.list_elements.row_count();
        let mut rng = rand::thread_rng();
        let rolled_element = rng.gen_range(0..elements_count);

        let all_rolls = self.list_elements.as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap();

        all_rolls.row_data(rolled_element).unwrap().clone()
    }

    pub fn check_name_and_roll(&self, list_name: &slint::SharedString) -> Option<slint::SharedString> {
        if *list_name == self.list_name {
            return Some(self.make_roll())
        }
        None
    }
}

impl SpecialDice {
    pub fn check_roll(&self, dices: &[i32]) -> Option<i32> {
        if self.state == "=" {
            if dices.len() < 2 {
                return Some(self.condition_id);
            }

            let mut pr_elem = dices[0];
            for dice in &dices[1..] {
                if *dice != pr_elem {
                    return None
                }
                pr_elem = *dice;
            }

            return Some(self.condition_id);
        }
        None
    }
}

impl Condition {
    pub fn call_condition(&self, field_adapter: FieldAdapter, info_panel_adapter: InfoPanelAdapter) {
        if self.rule.starts_with("list(") {
            let list_to_roll = &self.rule[5..(self.rule.len() - 1)];

            let lists = field_adapter.get_lists();
            for list in lists.iter() {
                let list_name: slint::SharedString = list_to_roll.into();
                let res = list.check_name_and_roll(&list_name);
                if let Some(rolled_value) = res {
                    info_panel_adapter.set_list_name(list_name);
                    info_panel_adapter.set_list_roll(rolled_value);
                    info_panel_adapter.set_panel_mode(4)
                }
            }
        }
    }

    pub fn id(&self) -> i32 {
        self.condition_id
    }
}
