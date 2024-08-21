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

        } else if self.state.starts_with("sum(") {
            if self.state.chars().nth(4).unwrap() == '<' {
                let possible_sum :i32 = self.state[5..self.state.len() - 1].parse().unwrap();

                if dices.iter().sum::<i32>() < possible_sum {
                    return Some(self.condition_id)
                }
            } else if self.state.chars().nth(4).unwrap() == '>' {
                let possible_sum :i32 = self.state[5..self.state.len() - 1].parse().unwrap();

                if dices.iter().sum::<i32>() > possible_sum {
                  return Some(self.condition_id)
                }
            } else {
                let possible_sum :i32 = self.state[4..self.state.len() - 1].parse().unwrap();

                if dices.iter().sum::<i32>() == possible_sum {
                    return Some(self.condition_id)
                }
            }

        } else if self.state.starts_with("diff(") {
            if dices.len() < 2 {
                return None;
            }

            if self.state.chars().nth(5).unwrap() == '<' {
                let possible_diff :i32 = self.state[6..self.state.len() - 1].parse().unwrap();

                let min_elem = dices.iter().min().unwrap();
                let max_elem = dices.iter().max().unwrap();

                if *max_elem - *min_elem < possible_diff {
                    return Some(self.condition_id)
                }
            } else if self.state.chars().nth(5).unwrap() == '>' {
                let possible_diff :i32 = self.state[6..self.state.len() - 1].parse().unwrap();

                let min_elem = dices.iter().min().unwrap();
                let max_elem = dices.iter().max().unwrap();

                if *max_elem - *min_elem > possible_diff {
                    return Some(self.condition_id)
                }
            } else {
                let possible_diff :i32 = self.state[5..self.state.len() - 1].parse().unwrap();

                let min_elem = dices.iter().min().unwrap();
                let max_elem = dices.iter().max().unwrap();

                if *max_elem - *min_elem == possible_diff {
                    return Some(self.condition_id)
                }
            }
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
