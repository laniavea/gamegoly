from typing import List, Optional, Tuple

class ListInfo:
    name: str
    rollable_objects: List[str]
    def __init__(self, list_name: str, rollable_objects: List[str]):
        self.name = list_name
        self.rollable_objects = rollable_objects

    def get_object_info(self, id_to_get: int) -> Optional[Tuple[Optional[int], str]]:
        if id_to_get >= len(self.rollable_objects):
            return None

        rollable_object = self.rollable_objects[id_to_get]

        if len(rollable_object) < 3:
            return (None, self.rollable_objects[id_to_get])

        condition_id: Optional[int] = None
        object_text = rollable_object

        if rollable_object[0] == "[":
            close_bracket = rollable_object.find("]")
            if close_bracket != -1 and close_bracket > 1:
                condition_id = int(rollable_object[1:close_bracket])
                if close_bracket + 1 != len(rollable_object):
                    object_text = rollable_object[close_bracket+1:]
                else:
                    object_text = ""

        return (condition_id, object_text)
