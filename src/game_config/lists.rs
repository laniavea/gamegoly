use crate::game_config::RawListInfo;

pub struct ListInfo {
    name: String,
    elements: Vec<String>,
    related_conditions: Vec<Option<usize>>
}

impl ListInfo {
    pub fn new(mut raw_list_info: RawListInfo) -> ListInfo {
        let related_conditions: Vec<Option<usize>> = Vec::new();
        for now_element in raw_list_info.elements.iter_mut() {
            *now_element = now_element.trim().to_string();

            let mut have_condition: bool = false;
            for now_char in now_element.chars() {
                if now_char == '[' {
                    have_condition = true
                }

                if have_condition {

                }
            }
        }

        ListInfo {
            name: raw_list_info.name,
            elements: raw_list_info.elements,
            related_conditions: vec![]
        }
    }
}
