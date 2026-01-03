from typing import List, Tuple, Optional, Dict

from pydantic import ValidationError

from . import raw_config
from .. import entity

class GameConfig:
    title: str = ""
    base_dice: List[entity.Dice]
    help_info: List[Tuple[str, str]] = []
    lists: Dict[str, List[str]]
    conditions: Dict[int, entity.ConditionInfo]

    def __init__(self, raw_config_obj: raw_config.RawGameConfig):
        self.title = raw_config_obj.field.title.strip()
        if not self.title:
            raise ValueError("Title includes only spaces")

        self.base_dice = entity.create_dices(raw_config_obj.field.base_dice)

        if raw_config_obj.field.help_info:
            self.help_info = GameConfig.create_help_info(raw_config_obj.field.help_info)

        self.lists = GameConfig.init_lists(raw_config_obj.lists)
        self.conditions = GameConfig.init_conditions(raw_config_obj.conditions);

    @staticmethod
    def create_help_info(raw_help_info: List[str]) -> List[Tuple[str, str]]:
        help_info = []

        for help_str in raw_help_info:
            if not help_str.startswith("{{"): # }} just to save ident
                raise ValueError("Config's help rule must starts with '{{rule_name}}'")

            rule_name_sym_status: bool = False
            rule_name_end_id: Optional[int] = None

            for (help_char_id, help_char) in enumerate(help_str):
                if help_char_id < 2: # To skip {{ ;and to save ident }}
                    continue

                if help_char == '}':
                    if rule_name_sym_status:
                        rule_name_end_id = help_char_id - 1
                        break
                    else:
                        rule_name_sym_status = True
                elif rule_name_sym_status:
                    rule_name_sym_status = False
            else:
                raise ValueError("Config's help rule must starts with '{{rule_name}}'")

            rule_name = help_str[2:rule_name_end_id].strip()
            try:
                help_str[rule_name_end_id+2]
            except IndexError:
                raise ValueError("Config's help rule must be with next format '{{rule_name}}rule_desc'")

            rule_info = help_str[rule_name_end_id+2:].strip()

            if not rule_name:
                raise ValueError("Config's rule name is empty")
            if not rule_info:
                raise ValueError("Config's rule info is empty")

            help_info.append((rule_name, rule_info))
        return help_info
    
    @staticmethod
    def init_lists(raw_lists: List[raw_config.RawListInfo]) -> Dict[str, List[str]]:
        roll_lists: Dict[str, List[str]] = {}
        for raw_list in raw_lists:
            list_name = raw_list.name

            if not raw_list.elements:
                raise ValueError(f"List '{list_name}' is empty")

            roll_lists[list_name] = raw_list.elements
        return roll_lists

    @staticmethod
    def init_conditions(raw_conditions: List[raw_config.RawConditionsInfo]) -> Dict[int, entity.ConditionInfo]:
        conditions: Dict[int, entity.ConditionInfo] = {}

        for raw_condition in raw_conditions:
            condition_info = entity.ConditionInfo.from_string(raw_condition.id, raw_condition.rule)
            if raw_condition.id in conditions:
                print(f"Condition by id {raw_condition.id} was overrided")
            conditions[raw_condition.id] = condition_info

        return conditions


def create_config(file_path):
    try:
        raw_config_obj = raw_config.read_raw_config(file_path)
    except FileNotFoundError as e:
        print(f"Config not found by path: {file_path}")
        raise e
    except ValidationError as e:
        print(f"Config types are incorrect {e}")
        raise e

    gc = GameConfig(raw_config_obj)

    print(gc.__dict__)
    print(gc.base_dice[0].__dict__)
