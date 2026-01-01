import unittest

from gamegoly.game_config.full_config import GameConfig

class TestHelpInfoCreattion(unittest.TestCase):
    def test_help_info_ok(self):
        assert GameConfig.create_help_info(["{{1}}2"]) == [("1", "2")]
        assert GameConfig.create_help_info(["{{ 1 }}\t\t2 "]) == [("1", "2")]
        assert GameConfig.create_help_info(["{{ 1 }}\t\t2 ", "{{3}}   4\n\n"]) == [("1", "2"), ("3", "4")]
        assert GameConfig.create_help_info(["{{1}2345}}2"]) == [("1}2345", "2")]
        assert GameConfig.create_help_info(["{{1}23\n\t45}}2"]) == [("1}23\n\t45", "2")]

    def test_help_info_fail(self):
        invalid_inputs = [
            ["{{   \n }}2"],
            ["{{}}2"],
            ["{{}}"],
            ["{{123}}"],
            ["{{\n123}}\t\t"],
            ["{{123}}\n\n"],
            ["123}}234"],
            ["{123}}234"],
            ["{{123}45}234"],
        ]

        for data in invalid_inputs:
            with self.subTest(data=data):
                with self.assertRaises(ValueError):
                    GameConfig.create_help_info(data)


