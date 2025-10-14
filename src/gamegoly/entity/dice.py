import random
import unittest

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
        for now_value in norm_dice_str.split(','):
            try:
                now_value_num = int(now_value)
            except ValueError as e:
                raise ValueError(f"Dice can't be parsed: {e}")

            if min_roll_value is None:
                min_roll_value = now_value_num
            elif max_roll_value is None:
                max_roll_value = now_value_num
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
    for (now_dice_id, now_dice_info) in enumerate(norm_dices_info.split()):
        try:
            now_dice = Dice(now_dice_info)
        except ValueError as e:
            raise ValueError(f"Failed to create dice #{now_dice_id+1}: {e}")

        all_dices.append(now_dice)

    if not all_dices:
        raise ValueError(f"Dices part doesn't contains any data")

    return all_dices

class TestDiceCreattion(unittest.TestCase):
    def test_dices_ok(self):
        dices = create_dices("1,6 4,5 -10,7 7,7");
        assert len(dices) == 4
        assert dices[0].min_roll_value == 1 and dices[0].max_roll_value == 6
        assert dices[1].min_roll_value == 4 and dices[1].max_roll_value == 5
        assert dices[2].min_roll_value == -10 and dices[2].max_roll_value == 7
        assert dices[3].min_roll_value == 7 and dices[3].max_roll_value == 7

    def test_dices_not_valid(self):
        with self.assertRaises(ValueError):
            create_dices("7,6")

    def test_dices_not_invalid_data(self):
        with self.assertRaises(ValueError):
            create_dices("7,")
        with self.assertRaises(ValueError):
            create_dices(",7")
        with self.assertRaises(ValueError):
            create_dices("aa")
        with self.assertRaises(ValueError):
            create_dices("")
        with self.assertRaises(ValueError):
            self.fail(create_dices("1, 3 "))
        with self.assertRaises(ValueError):
            self.fail(create_dices("1,3 4,6.1"))

if __name__ == "__main__":
    unittest.main()

