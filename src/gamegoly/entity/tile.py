from dataclasses import dataclass
from typing import Dict, Tuple, List

from ..game_config.raw_config import RawTilesInfo

@dataclass
class TileInfo:
    num_position: int
    title: str
    description: str
    rules: Dict[str, str]
    color: Tuple[int, int, int]
    condition_id: int

    @staticmethod
    def from_raw_tile(raw_tile_info: RawTilesInfo) -> TileInfo:
        num_position = raw_tile_info.pos
        title = raw_tile_info.title
        description = raw_tile_info.description
        condition_id = raw_tile_info.condition_id

        rules = TileInfo._parse_rules(raw_tile_info.rules)
        color = TileInfo._parse_color(raw_tile_info.color.strip())


        tiles_info = TileInfo(
                num_position=num_position,
                title=title,
                description=description,
                rules=rules,
                color=color,
                condition_id=condition_id,
        )

        return tiles_info

    @staticmethod
    def _parse_rules(raw_rules: List[str]) -> Dict[str, str]:
        rules = {}
        for rule in raw_rules:
            open_bracket_idx = rule.find('(') # just to save ident )
            close_bracket_idx = rule.find(')')

            if open_bracket_idx == -1:
                raise ValueError(f"Open bracket for rule's name '{rule}' not found")
            if close_bracket_idx == -1:
                raise ValueError(f"Close bracket for rule's name '{rule}' not found")

            if open_bracket_idx+1 >= close_bracket_idx or close_bracket_idx + 1 == len(rule):
                raise ValueError("Rules must be like '(rule_name)rule_value' " + f"but got '{rule}'")

            rule_name = rule[open_bracket_idx+1:close_bracket_idx].strip()
            rule_value = rule[close_bracket_idx+1:].strip()

            if not rule_name:
                raise ValueError("Rule name is empty")
            if not rule_value:
                raise ValueError("Rule value is empty")

            rules[rule_name] = rule_value

        return rules

    @staticmethod
    def _parse_color(raw_color_str: str) -> Tuple[int, int, int]:
        try:
            raw_color = [int(i.strip()) for i in raw_color_str.split(',')]
        except ValueError:
            raise ValueError(f"Color must be 3 numbers separated by ',' ex. '127,127,127'. Got - {raw_color_str}")

        if len(raw_color) != 3:
            raise ValueError(f"Color must be 3 numbers separated by ',' ex. '127,127,127'. Got - {raw_color_str}")

        color = (raw_color[0], raw_color[1], raw_color[2])

        return color


    def __post_init__(self):
        self.title = self.title.strip()
        if not self.title:
            raise ValueError("Title is empty")

        self.description = self.description.strip()
        if not self.description:
            raise ValueError("Description is empty")

        if not (0 <= self.color[0] <= 255):
            raise ValueError(f"First RGB color spec if out of bound: {self.color}")
        if not (0 <= self.color[1] <= 255):
            raise ValueError(f"Second RGB color spec if out of bound: {self.color}")
        if not (0 <= self.color[2] <= 255):
            raise ValueError(f"Third RGB color spec if out of bound: {self.color}")


