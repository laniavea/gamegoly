import tomllib

from typing import List, Tuple

from .. import entity

class GameConfig:
    title: str = ""
    base_dice: List[entity.Dice]
    raw_base_dice: str = ""
    help_info: List[Tuple[str, str]] = []

    def __init__(self, field_info):
        if field_info["title"] is None:
            raise ValueError("Field part doesn't contains 'title' value")
        self.title = field_info["title"]

        if field_info["base_dice"] is None:
            raise ValueError("Field part doesn't contains 'base_dice' value")
        self.raw_base_dice = field_info["base_dice"]
        try:
            self.base_dice = entity.create_dices(self.raw_base_dice)
        except ValueError as e:
            raise ValueError(f"Failed to create game config: {e}")

        if field_info["help_info"] is not None:
            self.rules = field_info["help_info"]


def create_config(file_path):
    config_raw_text = None
    with open(file_path, "rb") as f:
        config_raw_text = tomllib.load(f)

    if config_raw_text is None:
        raise ValueError("Incorrect config")

    field_raw_part = config_raw_text["field"]
    if field_raw_part is None:
        raise ValueError("Config doesn't contains Field part which is necessary")

    gc = GameConfig(field_raw_part)

    print(gc.__dict__)
    print(gc.base_dice[0].__dict__)
