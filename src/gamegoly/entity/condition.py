from dataclasses import dataclass
from typing import List, Literal, Union

@dataclass
class IntCondition:
    condition_type: Literal["skip_to_stage", "main_rule_from_tile", "move_to"]
    condition_value: int

@dataclass
class StrCondition:
    condition_type: Literal[
            "roll_list",
            "rand_main_rule",
            "main_rule_from_list",
            "main_rule_from_dist",
            "add_to_player_cubes",
            "change_player_cubes"
    ]
    condition_value: str

@dataclass
class IntListCondition:
    condition_type: Literal["spawn_conditions", "move_next"]
    condition_value: List[int]

@dataclass
class StrListCondition:
    condition_type: Literal["change_value"]
    condition_value: List[str]

@dataclass
class ConditionInfo:
    condition_id: int
    condition_desc: Union[IntCondition, StrCondition, IntListCondition, StrListCondition]

    @staticmethod
    def from_string(condition_id: int, condition_str: str) -> ConditionInfo:
        open_bracket_idx = condition_str.find('(') # just to save ident )
        close_bracket_idx = condition_str.find(')')

        if open_bracket_idx == -1:
            raise ValueError(f"Open bracket for condition '{condition_str}' not found")
        if close_bracket_idx == -1:
            raise ValueError(f"Close bracket for condition '{condition_str}' not found")

        if close_bracket_idx < open_bracket_idx:
            raise ValueError(f"Condition must be in next format 'cond_type(cond_rule)' but got '{condition_str}'")

        condition_type_str = condition_str[:open_bracket_idx]
        condition_value_str = condition_str[open_bracket_idx+1:close_bracket_idx]

        condition_desc: Union[IntCondition, StrCondition, IntListCondition, StrListCondition]

        match condition_type_str:
            case "roll_list":
                condition_desc = StrCondition("roll_list", condition_value_str)

            case "spawn_conditions":
                try:
                    conditions_to_spawn: List[int] = list(map(int, condition_value_str.split(',')))
                except ValueError as e:
                    raise ValueError(f"Error while parsing condition '{condition_str}': {e}")
                condition_desc = IntListCondition("spawn_conditions", conditions_to_spawn)
            case _:
                raise ValueError("Other conditions not implemented yet")

        return ConditionInfo(condition_id=condition_id, condition_desc=condition_desc)
