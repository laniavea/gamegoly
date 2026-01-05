from dataclasses import dataclass
from typing import List, Literal, Union

@dataclass
class NoParamCondition:
    condition_type: Literal["rand_main_rule"]

@dataclass
class IntCondition:
    condition_type: Literal["skip_to_stage", "move_to"]
    condition_value: int

@dataclass
class StrCondition:
    condition_type: Literal[
            "roll_list",
            "main_rule_from_tile_rules",
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
    condition_desc: Union[IntCondition, StrCondition, IntListCondition, StrListCondition, NoParamCondition]

    @staticmethod
    def from_string(condition_id: int, condition_str: str) -> ConditionInfo:
        condition_desc: Union[IntCondition, StrCondition, IntListCondition, StrListCondition, NoParamCondition]

        condition_str = condition_str.strip()
        if condition_str == "rand_main_rule":
            condition_desc = NoParamCondition(condition_type="rand_main_rule")
            return ConditionInfo(condition_id=condition_id, condition_desc=condition_desc)

        open_bracket_idx = condition_str.find('(') # just to save ident )
        close_bracket_idx = condition_str.find(')')

        if open_bracket_idx == -1:
            raise ValueError(f"Open bracket for condition '{condition_str}' not found")
        if close_bracket_idx == -1:
            raise ValueError(f"Close bracket for condition '{condition_str}' not found")

        if close_bracket_idx < open_bracket_idx:
            raise ValueError(f"Condition must be in next format 'cond_type(cond_rule)' but got '{condition_str}'")

        condition_type_str = condition_str[:open_bracket_idx].strip()
        condition_value_str = condition_str[open_bracket_idx+1:close_bracket_idx].strip()


        match condition_type_str:
            case "skip_to_stage" | "move_to":
                try:
                    condition_desc = IntCondition(condition_type_str, int(condition_value_str))
                except ValueError as e:
                    error_str = f"Error while parsing condition '{condition_str}'"
                    match condition_type_str:
                        case "skip_to_stage":
                            error_description = "Must contain ID of stage between 1 and 5; Ex. skip_to_stage(3)"
                        case "move_to":
                            error_description = "Must contain tile ID; Ex. move_to(10)"

                    raise ValueError(f"{error_str}. {error_description};\n{e}")

            case "roll_list" | "main_rule_from_list" | "main_rule_from_dist" | "main_rule_from_tile_rules" | "add_to_player_cubes" | "change_player_cubes":
                condition_desc = StrCondition(condition_type_str, condition_value_str)

            case "spawn_conditions" | "move_next":
                try:
                    int_values: List[int] = [int(x.strip()) for x in condition_value_str.split(',')]
                except ValueError as e:
                    error_str = f"Error while parsing condition '{condition_str}'"
                    match condition_type_str:
                        case "spawn_conditions":
                            error_description = "Must contain IDs of conditions to spawn; Ex. spawn_conditions(1, 1, 2)"
                        case "move_next":
                            error_description = "Must contain IDs of tiles to move next; Ex. move_next(5, 10, 15, 20)"

                    raise ValueError(f"{error_str}. {error_description};\n{e}")
                condition_desc = IntListCondition(condition_type_str, int_values)
            
            case "change_value":
                try:
                    str_values: List[str] = [x.strip() for x in condition_value_str.split(',')]
                except ValueError as e:
                    error_str = f"Error while parsing condition '{condition_str}'"
                    match condition_type_str:
                        case "change_value":
                            error_description = "Must contain str info about value change; Ex. change_value(drops, +2)"

                    raise ValueError(f"{error_str}. {error_description};\n{e}")
                condition_desc = StrListCondition(condition_type_str, str_values)
            case _:
                raise ValueError("Other conditions not implemented yet")

        return ConditionInfo(condition_id=condition_id, condition_desc=condition_desc)
