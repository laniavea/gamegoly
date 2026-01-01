import unittest

from gamegoly.entity.dice import Dice, create_dices

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
        invalid_inputs = [
            "7,",
            ",7",
            "aa",
            "",
            "1, 3 ",
            "1,3 4,6.1",
        ]

        for data in invalid_inputs:
            with self.subTest(data=data):
                with self.assertRaises(ValueError):
                    create_dices(data)
