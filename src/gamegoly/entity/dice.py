import random

from typing import List

class Dice:
    min_roll_value: int = 1
    max_roll_value: int = 6

    def __init__(self, dice_str: str): 
        norm_dice_str = dice_str.strip()
        if not norm_dice_str:
            raise ValueError("Dice description is empty")

        if ',' not in norm_dice_str:
            raise ValueError("Dice description doesn't contain splitter ',' between min and max values")

        min_roll_value = None
        max_roll_value = None
        for dice_value_str in norm_dice_str.split(','):
            try:
                dice_value_int = int(dice_value_str)
            except ValueError as e:
                raise ValueError(f"Dice can't be parsed: {e}")

            if min_roll_value is None:
                min_roll_value = dice_value_int
            elif max_roll_value is None:
                max_roll_value = dice_value_int
            else:
                raise ValueError("Too many arguments to create Dice, must be like '1,6' to create dice with 1-6 values")

        assert min_roll_value is not None
        assert max_roll_value is not None

        if min_roll_value > max_roll_value:
            raise ValueError("Dice min value is bigger than max value")

        self.min_roll_value = min_roll_value
        self.max_roll_value = max_roll_value

    def roll_dice(self) -> int:
        return random.randint(self.min_roll_value, self.max_roll_value)

def create_dices(dices_info: str) -> List[Dice]:
    norm_dices_info = dices_info.strip()

    all_dices = []
    for (dice_id, dice_info) in enumerate(norm_dices_info.split()):
        try:
            dice = Dice(dice_info)
        except ValueError as e:
            raise ValueError(f"Failed to create dice #{dice_id+1}: {e}")

        all_dices.append(dice)

    if not all_dices:
        raise ValueError(f"Dices part doesn't contains any data")

    return all_dices
