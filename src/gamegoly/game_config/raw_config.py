import tomllib
from typing import List, Optional

from pydantic import BaseModel

class RawFieldInfo(BaseModel):
    title: str
    base_dice: str
    help_info: Optional[List[str]]

class RawListInfo(BaseModel):
    name: str
    elements: List[str]

class RawConditionsInfo(BaseModel):
    id: int
    rule: str

class RawEventsInfo(BaseModel):
    state: str
    condition_id: int

class RawTilesInfo(BaseModel):
    pos: int
    title: str
    description: str
    rules: List[str]
    color: str
    condition_id: int

class RawGameConfig(BaseModel):
    field: RawFieldInfo
    lists: List[RawListInfo]
    conditions: List[RawConditionsInfo]
    events: List[RawEventsInfo]
    tiles: List[RawTilesInfo]

def read_raw_config(file_path: str) -> RawGameConfig:
    with open(file_path, "rb") as f:
        config_raw_toml = tomllib.load(f)

    raw_config_data: RawGameConfig = RawGameConfig(**config_raw_toml)

    return raw_config_data
