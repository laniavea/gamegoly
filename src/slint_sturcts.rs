use crate::{SpecialDice, Condition};

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
    pub fn check_condition(&self, orig_condition: String) {

    }
}
