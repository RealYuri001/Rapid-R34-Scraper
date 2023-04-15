import json
from typing import Generator

try:
    import orjson #type: ignore
except ImportError:
    HAS_ORJSON = False
else:
    HAS_ORJSON = True
    
if HAS_ORJSON:
    with open("fetch.json", "rb") as f:
        data = orjson.loads(f.read())
else:
    with open("fetch.json", "r") as f:
        data = json.load(f)

#I love type annotations.

def unpack_loops(data: list[dict]) -> dict:
    for dat in data:
        return dat

def get_useful_data_from_json_sample(data: dict) -> tuple[int, str, list[str], str, str]:
    tag: str = data.get("tags")
    id: int = data.get("id")
    
    tags: list[str] = tag.split(" ") #I don't know why they don't use array. What an ugly way.
    rate: str = data.get("rating") #Actually it's not that useful, just only for getting a rating. (Might be inaccurate)
    owner: str = data.get("owner")
    
    original_source: str = data.get("source") #Might get the forbidden error in some webs.
    
    return (id, owner, tags, rate, original_source)

print(get_useful_data_from_json_sample(unpack_loops(data)))