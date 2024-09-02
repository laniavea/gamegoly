use rand::Rng;
use slint::{Model, VecModel};

use crate::utils;
use crate::slint_callbacks::update_player_pos;
use crate::{FieldAdapter, InfoPanelAdapter, LowerPanelAdapter, SpecialDice, Condition, ListData, DiceRoll};

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
    pub fn call_condition(&self,
        field_adapter: &FieldAdapter,
        info_panel_adapter: &InfoPanelAdapter,
        lower_panel_adapter: &LowerPanelAdapter
    ) {
        if self.rule.starts_with("list(") {
            let list_to_roll = &self.rule[5..(self.rule.len() - 1)];

            let lists = field_adapter.get_lists();
            for list in lists.iter() {
                let list_name: slint::SharedString = list_to_roll.into();
                let res = list.check_name_and_roll(&list_name);
                if let Some(mut rolled_value) = res {
                    
                    if let Some((cond_id, new_rolled_value)) = utils::check_list_for_cond(rolled_value.as_str()) {
                        rolled_value = new_rolled_value;
                        field_adapter.get_conditions_queue()
                            .as_any().downcast_ref::<VecModel<i32>>().unwrap().push(cond_id);
                    } else {
                        match list_to_roll {
                            "Дополнительный тег" => {
                                let add_tags = lower_panel_adapter.get_player_add_tags();
                                add_tags.as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap().push(rolled_value.clone());
                                lower_panel_adapter.set_combined_add_tags(utils::combine_strings(lower_panel_adapter.get_player_add_tags()));
                            },
                            "Спешл" => {
                                let specials = lower_panel_adapter.get_player_special();
                                specials.as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap().push(rolled_value.clone());
                                lower_panel_adapter.set_combined_specials(utils::combine_strings(lower_panel_adapter.get_player_special()));
                            },
                            _ => ()
                        }
                    }

                    info_panel_adapter.set_list_name(list_name);
                    info_panel_adapter.set_list_roll(rolled_value);
                    info_panel_adapter.set_panel_mode(4);

                    break;
                }
            }

        } else if self.rule.starts_with("cond(") {
            let list_of_conds: Vec<i32> = self.rule[5..self.rule.len()-1].split(',')
                .map(|now_value| now_value.parse().unwrap()).collect();

            for now_cond in &list_of_conds {
                field_adapter.get_conditions_queue().as_any().downcast_ref::<VecModel<i32>>().unwrap().push(*now_cond);
            }

            info_panel_adapter.set_any_header(slint::SharedString::from("New events"));
            info_panel_adapter.set_any_text(slint::SharedString::from(format!("Added {} new events", list_of_conds.len())));
            info_panel_adapter.set_panel_mode(3)

        } else if self.rule.starts_with("ch_val(") {
            let data: Vec<&str> = self.rule[7..self.rule.len() - 1].split(',').collect();
            if data.len() == 2 {
                let val_name_to_change = data[0];
                let change_type = data[1].chars().nth(0).unwrap();
                let change_num: &str = &data[1][1..];

                let mut value_to_change: i32 = match val_name_to_change {
                    "drops" => field_adapter.get_player_drops(),
                    "half-moves" => field_adapter.get_player_half_moves(),
                    _ => {
                        eprintln!("Value to change must be drops or half-moves");
                        return
                    }
                };

                match change_type {
                    '+' => {
                        value_to_change += change_num.parse::<i32>().unwrap()
                    }
                    '-' => {
                        value_to_change -= change_num.parse::<i32>().unwrap()
                    }
                    '=' => {
                        value_to_change = change_num.parse().unwrap()
                    }
                    _ => {
                        eprintln!("Write ch_val as ch_val(var_to_change,[+-=]any_num)");
                        return
                    }
                }

                match val_name_to_change {
                    "drops" => field_adapter.set_player_drops(value_to_change),
                    "half-moves" => field_adapter.set_player_half_moves(value_to_change),
                    _ => {
                        unreachable!();
                    }
                };
                info_panel_adapter.set_any_header(slint::SharedString::from("Some stat was edited"));
                info_panel_adapter.set_any_text(slint::SharedString::from(format!("{val_name_to_change}: {}", data[1])));
                info_panel_adapter.set_panel_mode(3)
            }

        } else if self.rule.starts_with("rand_by(") {
            let elements_to_search: &str = &self.rule[8..self.rule.len() - 1];

            let player_loc = field_adapter.get_player_loc_id() as usize;
            let now_cond = utils::get_tile_data_from_tile_id(player_loc, field_adapter);

            let mut now_elements: Vec<slint::SharedString> = vec![];

            for rule in now_cond.rules.iter() {
                if let Some(now_rule) = rule.strip_prefix(elements_to_search) {
                    for elem in now_rule.split(",").map(|el| el.trim()) {
                        now_elements.push(slint::SharedString::from(elem));
                    }
                }
            }

            if now_elements.is_empty() {
                now_elements.push(slint::SharedString::from("Nothing"));
            }

            info_panel_adapter.set_rules_roll_list(slint::ModelRc::new(slint::VecModel::from(now_elements)));
            info_panel_adapter.set_panel_mode(5);

        } else if self.rule.starts_with("mt_list(") {
            let list_to_roll = &self.rule[8..(self.rule.len() - 1)];

            let lists = field_adapter.get_lists();
            for list in lists.iter() {
                let list_name: slint::SharedString = list_to_roll.into();
                let res = list.check_name_and_roll(&list_name);
                if let Some(rolled_value) = res {
                    info_panel_adapter.set_list_name(list_name);
                    info_panel_adapter.set_list_roll(rolled_value.clone());
                    info_panel_adapter.set_panel_mode(4);

                    lower_panel_adapter.set_player_main_tag(rolled_value);
                    lower_panel_adapter.set_player_status(3);
                    break;
                }
            }

        } else if self.rule.starts_with("skip(") {
            let new_player_status: i32 = self.rule[5..6].parse().unwrap();

            lower_panel_adapter.set_player_main_tag(slint::SharedString::from("None"));
            lower_panel_adapter.set_player_status(new_player_status);

            info_panel_adapter.set_any_header(slint::SharedString::from("Player status was updated"));
            info_panel_adapter.set_any_text(slint::SharedString::from(""));
            info_panel_adapter.set_panel_mode(3)

        } else if self.rule.starts_with("mt_from_rule(") {
            let rule_id: usize = self.rule[13..14].parse().unwrap();

            let player_loc = field_adapter.get_player_loc_id() as usize;
            let now_tile = utils::get_tile_data_from_tile_id(player_loc, field_adapter);

            let rule = now_tile.rules.as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap()
                .row_data(rule_id-1).unwrap_or(slint::SharedString::from("None"));

            lower_panel_adapter.set_player_main_tag(rule);
            lower_panel_adapter.set_player_status(3);

            info_panel_adapter.set_any_header(slint::SharedString::from("Player's main tag was updated"));
            info_panel_adapter.set_any_text(slint::SharedString::from(""));
            info_panel_adapter.set_panel_mode(3);

        } else if self.rule.starts_with("rand_by_dist(") {
            let rule_content = &self.rule[13..self.rule.len()-1];

            let mut rules: Vec<&str> = vec![];
            let mut rules_weight: Vec<i32> = vec![];

            for now_rule in rule_content.split(',') {
                let rule_and_weight: Vec<&str> = now_rule.split('(').collect();

                if rule_and_weight.len() == 2{
                    rules.push(rule_and_weight[0]);
                    rules_weight.push(rule_and_weight[1][..rule_and_weight[1].len()-1].parse::<i32>().unwrap());
                }
            }
            let mut rng = rand::thread_rng();
            let rolled_weight = rng.gen_range(0..rules_weight.iter().sum());

            let mut temp_sum = 0;
            for (now_id, now_weight) in rules_weight.iter().enumerate() {
                temp_sum += now_weight;
                if temp_sum > rolled_weight {
                    let rolled_value = slint::SharedString::from(rules[now_id]);

                    lower_panel_adapter.set_player_main_tag(rolled_value);
                    lower_panel_adapter.set_player_status(3);

                    info_panel_adapter.set_any_header(slint::SharedString::from("Player's main tag was updated"));
                    info_panel_adapter.set_any_text(slint::SharedString::from(""));
                    info_panel_adapter.set_panel_mode(3);
                    return
                } 
            }
            unreachable!();

        } else if self.rule.starts_with("pl_cube_add(") {
            let add_dice = &self.rule[12..self.rule.len()-1];

            let new_dice = utils::dices_from_string(add_dice).unwrap();
            let add_dices = field_adapter.get_add_dice();

            add_dices.as_any().downcast_ref::<VecModel<DiceRoll>>().unwrap().extend(new_dice);

        } else if self.rule.starts_with("pl_cube_set(") {
            let over_dice = &self.rule[12..self.rule.len()-1];

            let new_dice = utils::dices_from_string(over_dice).unwrap();
            let override_dices = field_adapter.get_override_dice();

            override_dices.as_any().downcast_ref::<VecModel<DiceRoll>>().unwrap().extend(new_dice);

        } else if self.rule.starts_with("mv(") {
            let move_id = self.rule[3..self.rule.len()-1].parse::<i32>().unwrap();

            update_player_pos(field_adapter, move_id)

        } else if self.rule.starts_with("mv_next(") {
            let move_ids = &self.rule[8..self.rule.len()-1];

            let move_ids_num: Vec<i32> = move_ids.split(',').map(|val| val.parse::<i32>().unwrap()).collect();
            let player_pos = field_adapter.get_player_loc_id();

            for now_move_id in &move_ids_num {
                if player_pos < *now_move_id {
                    update_player_pos(field_adapter, *now_move_id);
                    return
                }
            }
            update_player_pos(field_adapter, move_ids_num[0]);
        }
    }

    pub fn id(&self) -> i32 {
        self.condition_id
    }
}
