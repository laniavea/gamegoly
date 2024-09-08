use crate::utils;
use crate::config_player::serialize_player;
use crate::{AppWindow, Condition, FieldTilesData};
use crate::{FieldAdapter, LowerPanelAdapter, InfoPanelAdapter};

use slint::Weak;
use slint::{Model, VecModel};
use slint::ComponentHandle;

use rand::Rng;

pub fn lower_panel_callbacks(window: Weak<AppWindow>) {
    let main_window = window.unwrap();

    // +1 turn
    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_update_player_state(move |player_loc| {
        let new_main_window = main_window_weak.unwrap();
        let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();
        let field_adapter = new_main_window.global::<FieldAdapter>();

        update_player_pos(&field_adapter, player_loc);
        lower_panel_adapter.set_player_status(2);
    });

    // Dice roll
    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_roll_dice(move || {
        let new_main_window = main_window_weak.unwrap();

        let field_adapter = new_main_window.global::<FieldAdapter>();
        let info_panel_adapter = new_main_window.global::<InfoPanelAdapter>();
        let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();

        let base_dices = field_adapter.get_base_dice();
        let override_dice = field_adapter.get_override_dice();
        let add_dice = field_adapter.get_add_dice();

        let dices = if override_dice.row_count() != 0 {
            field_adapter.set_override_dice(slint::ModelRc::new(slint::VecModel::from(Vec::new())));
            utils::roll_dices(override_dice)
        } else if add_dice.row_count() != 0 {
            field_adapter.set_add_dice(slint::ModelRc::new(slint::VecModel::from(Vec::new())));
            let mut base_dices = utils::roll_dices(base_dices);
            base_dices.extend(utils::roll_dices(add_dice));
            base_dices
        } else {
            utils::roll_dices(base_dices)
        };

        let mut dices_sum: i32 = 0;
        let mut dices_max_value: u32 = 0;
        let mut dices_max_value_is_pos: bool = true;

        for value in &dices {
            dices_sum += value;

            if value.unsigned_abs() > dices_max_value {
                dices_max_value = value.unsigned_abs();
                dices_max_value_is_pos = *value >= 0;
            }
        }

        let max_digits = (dices_max_value.checked_ilog10().unwrap_or(0) as i32) + 1;

        info_panel_adapter.set_dices_count(dices.len() as i32);
        info_panel_adapter.set_dices_max_digits(max_digits + if dices_max_value_is_pos {0} else {1});
        info_panel_adapter.set_dices(slint::ModelRc::new(slint::VecModel::from(dices.clone())));
        info_panel_adapter.set_panel_mode(2);

        let new_player_loc = dices_sum + field_adapter.get_player_loc_id();
        update_player_pos(&field_adapter, new_player_loc);

        let special_dices = field_adapter.get_special_dices();

        let condions_ids = utils::special_dices_check(&dices, special_dices);
        if !condions_ids.is_empty() {
            field_adapter.set_conditions_queue(slint::ModelRc::new(slint::VecModel::from(condions_ids)));
            let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();
            lower_panel_adapter.set_condition_button(true);
        }

        lower_panel_adapter.set_player_status(2);
    });

    // Save player state
    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_save_player_state(move || {
        let new_main_window = main_window_weak.unwrap();
        let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();
        let field_adapter = new_main_window.global::<FieldAdapter>();

        match serialize_player(lower_panel_adapter, field_adapter) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{}", err);
            }
        }

    });

    // Call condition button
    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_roll_next_condition(move || {
        let new_main_window = main_window_weak.unwrap();
        let field_adapter = new_main_window.global::<FieldAdapter>();

        let condition_offset: i32 = field_adapter.get_conditions_offset();
        let conditions_queue = field_adapter.get_conditions_queue();
        let conditions_queue = conditions_queue.as_any().downcast_ref::<VecModel<i32>>().unwrap();
        let conditions = field_adapter.get_conditions();
        let conditions = conditions.as_any().downcast_ref::<VecModel<Condition>>().unwrap();

        let now_codition_id = conditions_queue.row_data(condition_offset as usize).unwrap();
        for condition in conditions.iter() {
            if condition.id() == now_codition_id {
                let info_panel_adapter = new_main_window.global::<InfoPanelAdapter>();
                let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();
                condition.call_condition(&field_adapter, &info_panel_adapter, &lower_panel_adapter);
                break;
            }
        }

        let conditions_queue = field_adapter.get_conditions_queue();

        if condition_offset + 1 != conditions_queue.row_count() as i32 {
            field_adapter.set_conditions_offset(condition_offset + 1);
        } else {
            field_adapter.set_conditions_offset(0);
            field_adapter.set_conditions_queue(slint::ModelRc::new(slint::VecModel::from(vec![])));
            let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();
            lower_panel_adapter.set_condition_button(false);
        }
    });

    // Roll tag button
    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_roll_tag(move || {
        let new_main_window = main_window_weak.unwrap();
        let field_adapter = new_main_window.global::<FieldAdapter>();
        let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();

        let player_loc = field_adapter.get_player_loc_id() as usize;
        let cond: FieldTilesData = utils::get_tile_data_from_tile_id(player_loc, &field_adapter);

        field_adapter.set_conditions_queue(slint::ModelRc::new(slint::VecModel::from(vec![cond.condition_id])));
        lower_panel_adapter.set_condition_button(true);
    });

    // Roll number between
    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_roll_game(move || {
        let new_main_window = main_window_weak.unwrap();
        let info_panel_adapter = new_main_window.global::<InfoPanelAdapter>();

        let new_max = match info_panel_adapter.get_number_of_games() {
            0 => 100,
            value => {
                info_panel_adapter.set_number_of_games(0);
                value
            },
        };

        let new_roll_list: Vec<slint::SharedString> = vec![
            slint::SharedString::from("1"),
            slint::SharedString::from(format!("{}", new_max)),
            slint::SharedString::from(""),
            slint::SharedString::from(""),
            slint::SharedString::from(""),
        ];

        info_panel_adapter.set_max_value(new_max);
        info_panel_adapter.set_input_roll_list(slint::ModelRc::new(slint::VecModel::from(new_roll_list)));
        info_panel_adapter.set_panel_mode(6);
    });

    // Remove used information / stats
    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_commit_used(move || {
        let new_main_window = main_window_weak.unwrap();
        let info_panel_adapter = new_main_window.global::<InfoPanelAdapter>();
        let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();

        let specials_num = lower_panel_adapter.get_player_special()
            .as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap().row_count();

        let add_tags_num = lower_panel_adapter.get_player_add_tags()
            .as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap().row_count();

        info_panel_adapter.set_used_specials(slint::ModelRc::new(slint::VecModel::from((0..specials_num).map(|_| false).collect::<Vec<bool>>())));
        info_panel_adapter.set_used_add_tags(slint::ModelRc::new(slint::VecModel::from((0..add_tags_num).map(|_| false).collect::<Vec<bool>>())));

        info_panel_adapter.set_panel_mode(7);
    });

    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_complete_game(move || {
        let new_main_window = main_window_weak.unwrap();
        let info_panel_adapter = new_main_window.global::<InfoPanelAdapter>();

        info_panel_adapter.set_panel_mode(8);
    });
}

pub fn field_callbacks(window: Weak<AppWindow>) {
    let main_window = window.unwrap();

    // Roll random element from list
    main_window.global::<FieldAdapter>().on_roll_list_item(move |list_data| {
        list_data.make_roll()
    });
}

pub fn info_panel_callbacks(window: Weak<AppWindow>) {
    let main_window = window.unwrap();

    // Roll random main tag
    let main_window_weak = main_window.as_weak();
    main_window.global::<InfoPanelAdapter>().on_roll_main_tag(move || {
        let new_main_window = main_window_weak.unwrap();
        let info_panel_adapter = new_main_window.global::<InfoPanelAdapter>();

        let inputted_strings = info_panel_adapter.get_input_roll_list();

        let number_of_strings_to_parse = info_panel_adapter.get_rules_roll_list()
            .as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap().row_count();

        let inputted_nums = match utils::parse_vec_shared_str(inputted_strings, number_of_strings_to_parse) {
            Ok(nums) => { nums },
            Err(_) => {
                info_panel_adapter.set_roll_button_text(slint::SharedString::from("Error. Retype and try again"));
                return
            }
        };

        let rolled_tag_id = utils::roll_id_by_number_cummul(&inputted_nums);
        let main_tag = info_panel_adapter.get_rules_roll_list()
            .as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap()
            .row_data(rolled_tag_id).unwrap_or(slint::SharedString::from("None"));

        let main_tag_str = main_tag.clone().to_string();

        let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();
        lower_panel_adapter.set_player_main_tag(main_tag);
        lower_panel_adapter.set_player_status(3);

        info_panel_adapter.set_number_of_games(inputted_nums[rolled_tag_id]);

        info_panel_adapter.set_any_header(slint::SharedString::from("Main rule"));
        info_panel_adapter.set_any_text(slint::SharedString::from(format!("Your new main rule is: {}", main_tag_str)));
        info_panel_adapter.set_panel_mode(3);
    });

    // Roll random game's num
    let main_window_weak = main_window.as_weak();
    main_window.global::<InfoPanelAdapter>().on_roll_num_between(move || {
        let new_main_window = main_window_weak.unwrap();
        let info_panel_adapter = new_main_window.global::<InfoPanelAdapter>();

        let inputted_strings = info_panel_adapter.get_input_roll_list();

        let inputted_nums = match utils::parse_vec_shared_str(inputted_strings, 2) {
            Ok(nums) => { nums },
            Err(_) => {
                info_panel_adapter.set_roll_num_button_text(slint::SharedString::from("Error. Retype and try again"));
                info_panel_adapter.set_rolled_num_but_v(false);
                return
            }
        };

        let (lower_bound, upper_bound) = (inputted_nums[0], inputted_nums[1]);
        if lower_bound > upper_bound {
            info_panel_adapter.set_roll_num_button_text(slint::SharedString::from("Error. Retype and try again"));
            info_panel_adapter.set_rolled_num_but_v(false);
            return
        }

        info_panel_adapter.set_roll_num_button_text(slint::SharedString::from("Roll game's number"));

        let avg_value = (upper_bound + lower_bound) / 2;

        let mut rng = rand::thread_rng();
        let rolled_value = rng.gen_range(lower_bound..upper_bound+1);

        let alt_rolled_value = if rolled_value >= avg_value {
            rolled_value - avg_value
        } else {
            rolled_value + avg_value
        };

        info_panel_adapter.set_rolled_num_but_v(true);
        info_panel_adapter.set_rolled_num(slint::SharedString::from(format!("{rolled_value}")));
        info_panel_adapter.set_rolled_num_alt(slint::SharedString::from(format!("{alt_rolled_value}")));
        info_panel_adapter.set_panel_mode(6)

    });

    // Continue when game have been chosen
    let main_window_weak = main_window.as_weak();
    main_window.global::<InfoPanelAdapter>().on_to_state_4(move || {
        let new_main_window = main_window_weak.unwrap();
        let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();
        let info_panel_adapter =  new_main_window.global::<InfoPanelAdapter>();

        info_panel_adapter.set_any_header(slint::SharedString::from("You chosed your game!"));
        info_panel_adapter.set_any_text(slint::SharedString::from(""));
        info_panel_adapter.set_panel_mode(3);

        lower_panel_adapter.set_player_status(4);
    });

    // Commit used modifiers
    let main_window_weak = main_window.as_weak();
    main_window.global::<InfoPanelAdapter>().on_modifers_end(move || {
        let new_main_window = main_window_weak.unwrap();
        let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();
        let info_panel_adapter = new_main_window.global::<InfoPanelAdapter>();

        let used_specials = info_panel_adapter.get_used_specials();
        let used_add_tags = info_panel_adapter.get_used_add_tags();
        let specials_model = lower_panel_adapter.get_player_special();
        let add_tags_model = lower_panel_adapter.get_player_add_tags();

        let used_specials_vec = used_specials.as_any().downcast_ref::<VecModel<bool>>().unwrap();
        let used_add_tags_vec = used_add_tags.as_any().downcast_ref::<VecModel<bool>>().unwrap();
        let specials_vecmodel = specials_model.as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap();
        let add_tags_vecmodel = add_tags_model.as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap();
        
        let mut new_specials: Vec<slint::SharedString> = vec![];
        let mut new_add_tags: Vec<slint::SharedString> = vec![];

        for (now_sp_num, now_sp) in used_specials_vec.iter().enumerate() {
            if !now_sp {
                new_specials.push(specials_vecmodel.row_data(now_sp_num)
                    .unwrap_or(slint::SharedString::from("Error. Impossible to get")))
            }
        }

        for (now_add_tag_num, now_add_tag) in used_add_tags_vec.iter().enumerate() {
            if !now_add_tag {
                new_add_tags.push(add_tags_vecmodel.row_data(now_add_tag_num)
                    .unwrap_or(slint::SharedString::from("Error. Impossible to get")))
            }
        }

        let half_move_used: bool = info_panel_adapter.get_half_move_used();
        if half_move_used {
            let field_adapter = new_main_window.global::<FieldAdapter>();
            let half_moves = field_adapter.get_player_half_moves();
            field_adapter.set_player_half_moves(half_moves - 1);
        }

        lower_panel_adapter.set_player_special(slint::ModelRc::new(slint::VecModel::from(new_specials)));
        lower_panel_adapter.set_player_add_tags(slint::ModelRc::new(slint::VecModel::from(new_add_tags)));

        lower_panel_adapter.set_combined_specials(utils::combine_strings(lower_panel_adapter.get_player_special()));
        lower_panel_adapter.set_combined_add_tags(utils::combine_strings(lower_panel_adapter.get_player_add_tags()));

        info_panel_adapter.set_any_header(slint::SharedString::from("Status updated!"));
        info_panel_adapter.set_any_text(slint::SharedString::from(""));
        info_panel_adapter.set_panel_mode(3);

        lower_panel_adapter.set_player_status(5);
    });

    // Complete game button
    let main_window_weak = main_window.as_weak();
    main_window.global::<InfoPanelAdapter>().on_game_complete(move || {
        let new_main_window = main_window_weak.unwrap();
        let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();
        let info_panel_adapter = new_main_window.global::<InfoPanelAdapter>();

        lower_panel_adapter.set_player_main_tag(slint::SharedString::from("None"));
        lower_panel_adapter.set_player_status(1);

        info_panel_adapter.set_any_header(slint::SharedString::from("Status updated!"));
        info_panel_adapter.set_any_text(slint::SharedString::from("Game completed"));
        info_panel_adapter.set_panel_mode(3);
    });

    // Drop game button
    let main_window_weak = main_window.as_weak();
    main_window.global::<InfoPanelAdapter>().on_game_dropped(move || {
        let new_main_window = main_window_weak.unwrap();
        let lower_panel_adapter = new_main_window.global::<LowerPanelAdapter>();
        let info_panel_adapter = new_main_window.global::<InfoPanelAdapter>();
        let field_adapter = new_main_window.global::<FieldAdapter>();

        lower_panel_adapter.set_player_main_tag(slint::SharedString::from("None"));
        lower_panel_adapter.set_player_status(1);

        let now_drops_num = field_adapter.get_player_drops();

        if now_drops_num == 0 {
            update_player_pos(&field_adapter, 10);
            info_panel_adapter.set_any_header(slint::SharedString::from("JAIL!"));
            info_panel_adapter.set_any_text(slint::SharedString::from("You're in jail, roll by JAIL rules"));
        } else {
            field_adapter.set_player_drops(field_adapter.get_player_drops() - 1);
            info_panel_adapter.set_any_header(slint::SharedString::from("Status updated!"));
            info_panel_adapter.set_any_text(slint::SharedString::from("Game dropped"));
        }

        info_panel_adapter.set_panel_mode(3);

    });
}

pub fn update_player_pos(field_adapter: &FieldAdapter, player_loc: i32) {
    let number_of_tiles = field_adapter.get_number_of_tiles();
    if player_loc >= number_of_tiles {
        let player_drops = field_adapter.get_player_drops();
        field_adapter.set_player_drops(player_drops + 1);
    }
    let new_player_loc = player_loc % number_of_tiles;

    let (ver_state, hor_state) = utils::get_ver_hor_state(new_player_loc, number_of_tiles);

    field_adapter.set_player_on_ver(ver_state);
    field_adapter.set_player_on_hor(hor_state);
    field_adapter.set_player_loc_id(new_player_loc);
}
