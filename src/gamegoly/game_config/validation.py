import logging
from typing import List, Set, Dict, Optional

from . import full_config;
from .. import entity

def get_available_conditions(conditions: Dict[int, entity.ConditionInfo]) -> Set[int]:
    available_conditions: Set[int] = set(conditions.keys())
    return available_conditions

def validate_tiles(tiles: List[entity.TileInfo], conditions: Set[int]) -> List[entity.TileInfo]:
    """
    Validates a colletion of tiles and return them ordered by numeric position

    Checks:
        title; description; rules are not empty
        color - valid RGB set
        condition ID - exists in available conditions
        Num position - do not shared between multiple tiles

    Args:
        tiles: A list of TileInfo objects to validate
        conditions: A set of valid condition IDs that tiles may refer

    Return:
        Reordered TileInfo based on numeric position

    Raises:
        ValueError - if tile have invalid condition ID or if some of tiles try to share same num position
    """
    reordered_tiles: List[entity.TileInfo] = sorted(tiles, key=lambda x: x.num_position)

    pr_tile_num_position: Optional[int] = None
    for tile in tiles:
        if pr_tile_num_position is not None:
            if pr_tile_num_position == tile.num_position:
                logging.error(
                    f"Two tiles shares the same num position: {tile.num_position}"
                )
                raise ValueError(f"Invalid num position")
        pr_tile_num_position = tile.num_position

        if not tile.title:
            logging.warning(f"Tile with num: {tile.num_position} have empty title")
        if not tile.description:
            logging.warning(f"Tile with num: {tile.num_position} have empty description")
        for (color_id, color_value) in enumerate(tile.color):
            if color_value < 0 or color_value > 255:
                logging.error(
                    f"Tile with num: {tile.num_position} have invalid color in {color_id+1} param, must be in [0;255]"
                )
                raise ValueError("Invalid color")

        if not tile.rules:
            logging.warning(f"Tile with num: {tile.num_position} have no rules")
        if tile.condition_id not in conditions:
            logging.error(f"Tile with num: {tile.num_position} have unknown condition id")
            raise ValueError(f"Invalid condition id")

    return reordered_tiles

def validate_lists(lists: Dict[str, entity.ListInfo], conditions: Set[int]):
    for list_info in lists.values():
        if len(list_info.rollable_objects) == 0:
            logging.warning(f"List '{list_info.name}' do not contains any elements")

        condition_id = None
        for rollable_object_id in range(len(list_info.rollable_objects)):
            try:
                res = list_info.get_object_info(rollable_object_id)
                if res is not None:
                    condition_id = res[0]
            except ValueError:
                logging.error(f"Not an int for condition id in list's '{list_info.name}' element #{rollable_object_id}")
                raise ValueError("Incorrect list condition id")

            if condition_id is not None:
                if condition_id not in conditions:
                    logging.error(f"Unknown condition id in list's '{list_info.name}' element #{rollable_object_id}")
                    raise ValueError(f"Invalid condition id")


def validate_config(gc: full_config.GameConfig):
    available_conditions = get_available_conditions(gc.conditions)
    gc.tiles = validate_tiles(gc.tiles, available_conditions)
    validate_lists(gc.lists, available_conditions)
