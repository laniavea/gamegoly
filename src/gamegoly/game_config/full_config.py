from typing import List, Optional, Dict
import logging

from pydantic import ValidationError

from . import raw_config
from . import validation
from .. import api_handlers
from .. import entity

class GameConfig:
    title: str = ""
    base_dice: List[entity.Dice]
    help_info: List[entity.HelpInfo] = []
    lists: Dict[str, entity.ListInfo]
    conditions: Dict[int, entity.ConditionInfo]
    events: List[entity.EventInfo]
    tiles: List[entity.TileInfo]

    def __init__(self, raw_config_obj: raw_config.RawGameConfig):
        self.title = raw_config_obj.field.title.strip()
        if not self.title:
            raise ValueError("Title includes only spaces")

        self.base_dice = entity.create_dices(raw_config_obj.field.base_dice)

        if raw_config_obj.field.help_info:
            self.help_info = GameConfig.create_help_info(raw_config_obj.field.help_info)

        self.lists = GameConfig.init_lists(raw_config_obj.lists)
        self.conditions = GameConfig.init_conditions(raw_config_obj.conditions)
        self.events = GameConfig.init_events(raw_config_obj.events)
        self.tiles = GameConfig.init_tiles(raw_config_obj.tiles)

    @staticmethod
    def create_help_info(raw_help_info: List[str]) -> List[entity.HelpInfo]:
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

            help_info.append(entity.HelpInfo(name=rule_name, description=rule_info))
        return help_info
    
    @staticmethod
    def init_lists(raw_lists: List[raw_config.RawListInfo]) -> Dict[str, entity.ListInfo]:
        roll_lists: Dict[str, entity.ListInfo] = {}
        for raw_list in raw_lists:
            list_name = raw_list.name

            if list_name in roll_lists:
                logging.error(f"List with this name already exists: {list_name}")
                raise ValueError("Incorrect list name")

            if len(list_name) == 0:
                logging.error(f"One of the lists have empty name")
                raise ValueError("Incorrect list name")

            list_info = entity.ListInfo(list_name=list_name, rollable_objects=raw_list.elements)
            roll_lists[list_name] = list_info
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

    @staticmethod
    def init_events(raw_events: List[raw_config.RawEventsInfo]) -> List[entity.EventInfo]:
        events = []
        for raw_event in raw_events:
            events.append(entity.EventInfo.from_string(raw_event.condition_id, raw_event.state))

        return events

    @staticmethod
    def init_tiles(raw_tiles: List[raw_config.RawTilesInfo]) -> List[entity.TileInfo]:
        tiles = []
        for raw_tile in raw_tiles:
            tiles.append(entity.TileInfo.from_raw_tile(raw_tile))

        return tiles

    def get_tiles(self) -> List[api_handlers.TileInfoApi]:
        tiles = []
        for tile in self.tiles:
            color_str = f"rgb({tile.color[0]},{tile.color[1]},{tile.color[2]})"
            tiles.append(api_handlers.TileInfoApi(
                title = tile.title,
                description = tile.description,
                color = color_str,
            ))
        return tiles


def create_config(file_path) -> GameConfig:
    try:
        raw_config_obj = raw_config.read_raw_config(file_path)
    except FileNotFoundError as e:
        print(f"Config not found by path: {file_path}")
        raise e
    except ValidationError as e:
        print(f"Config types are incorrect {e}")
        raise e

    gc = GameConfig(raw_config_obj)
    validation.validate_config(gc)

    print(gc.__dict__)
    print(gc.base_dice[0].__dict__)

    return gc
