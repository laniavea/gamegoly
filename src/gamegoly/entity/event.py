from dataclasses import dataclass
from typing import List, Literal, Optional
from enum import Enum, auto

class CubeGroupFunctions(Enum):
    SUM = auto()
    DIFF = auto()
    SAME = auto()

class CubeGroupFuncSpec(Enum):
    MORE = auto()
    LESS = auto()
    EQUAL = auto()
    NA = auto()

@dataclass
class CubesEvent:
    event_trigger: Literal["cubes"]
    group_function: CubeGroupFunctions 
    group_func_spec: CubeGroupFuncSpec
    target_value: Optional[int]

    def __post_init__(self):
        if self.group_function == CubeGroupFunctions.SAME:
            if self.group_func_spec != CubeGroupFuncSpec.NA or self.target_value is not None:
                raise ValueError(f"Incorrect values for {self.group_function.name} cubes event")
        else:
            if self.group_func_spec == CubeGroupFuncSpec.NA or self.target_value is None:
                raise ValueError(f"Incorrect values for {self.group_function.name} cubes event")

    def is_triggered(self, cube_results: List[int]) -> bool:
        if not cube_results:
            raise ValueError("Got empty cube results to check")

        match self.group_function:
            case CubeGroupFunctions.SUM:
                value_to_check = sum(cube_results)
            case CubeGroupFunctions.DIFF:
                value_to_check = max(cube_results) - min(cube_results)
            case CubeGroupFunctions.SAME:
                return max(cube_results) == min(cube_results)

        if self.target_value is None:
            raise ValueError(f"CubesEvent got damaged. target value not specified")

        match self.group_func_spec:
            case CubeGroupFuncSpec.MORE:
                return value_to_check > self.target_value
            case CubeGroupFuncSpec.LESS:
                return value_to_check < self.target_value
            case CubeGroupFuncSpec.EQUAL:
                return value_to_check == self.target_value
            case CubeGroupFuncSpec.NA:
                raise ValueError(f"CubesEvent got damaged. NA not in SAME type of aggregate function")

    @staticmethod
    def from_string(event_condition_str: str) -> CubesEvent:
        open_bracket_idx = event_condition_str.find('(') # just to save ident )
        close_bracket_idx = event_condition_str.find(')')

        if event_condition_str == '=':
            return CubesEvent("cubes", CubeGroupFunctions.SAME ,CubeGroupFuncSpec.NA, None)

        if open_bracket_idx == -1:
            raise ValueError(f"Open bracket for event '{event_condition_str}' not found")
        if close_bracket_idx == -1:
            raise ValueError(f"Close bracket for event '{event_condition_str}' not found")

        if close_bracket_idx < open_bracket_idx:
            raise ValueError("Cubes event must be like 'func(value)' " + f"but got '{event_condition_str}'")

        event_condition_func = event_condition_str[:open_bracket_idx].strip()
        event_condition_val = event_condition_str[open_bracket_idx+1:close_bracket_idx].strip()

        match event_condition_func:
            case "sum":
                group_function = CubeGroupFunctions.SUM
            case "diff":
                group_function = CubeGroupFunctions.DIFF
            case _:
                raise ValueError(f"Unknown group function, got {event_condition_func}")

        if not event_condition_val:
            raise ValueError(f"Value after group function isn't specified, got {event_condition_str}")

        match event_condition_val[0]:
            case '>':
                group_func_spec = CubeGroupFuncSpec.MORE
                if len(event_condition_val) < 2:
                    raise ValueError(f"Value after group function isn't specified, got {event_condition_str}")
                target_value = int(event_condition_val[1:].strip())
            case '<':
                group_func_spec = CubeGroupFuncSpec.LESS
                if len(event_condition_val) < 2:
                    raise ValueError(f"Value after group function isn't specified, got {event_condition_str}")
                target_value = int(event_condition_val[1:].strip())
            case _:
                group_func_spec = CubeGroupFuncSpec.EQUAL
                target_value = int(event_condition_val)

        return CubesEvent(
                "cubes",
                group_function,
                group_func_spec,
                target_value
        )


@dataclass
class EventInfo:
    event_type: CubesEvent
    condition_to_spawn: int

    @staticmethod
    def from_string(condition_id: int, event_str: str) -> EventInfo:
        open_bracket_idx = event_str.find('{') # just to save ident }
        close_bracket_idx = event_str.find('}')

        if open_bracket_idx == -1:
            raise ValueError(f"Open bracket for event '{event_str}' not found")
        if close_bracket_idx == -1:
            raise ValueError(f"Close bracket for event '{event_str}' not found")

        if close_bracket_idx < open_bracket_idx:
            raise ValueError("Event must be in next format 'event_trigger{event_rule}'" + f"but got '{event_str}'")

        event_trigger_str = event_str[:open_bracket_idx].strip()
        event_rule_str = event_str[open_bracket_idx+1:close_bracket_idx].strip()

        match event_trigger_str:
            case 'cubes':
                try:
                    cubes_event = CubesEvent.from_string(event_rule_str)
                except ValueError as e:
                    raise ValueError(f"Error while parsing {event_str}: {e}")
            case _:
                raise ValueError(f"Unknown event type, got '{event_trigger_str}' from '{event_str}'")

        return EventInfo(condition_to_spawn=condition_id, event_type=cubes_event)

