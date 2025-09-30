# DONT USE

VERSION = "0.2.1"

locations = [
    ("./vec-utils/Cargo.toml", "package"),
    ("./vec-utils-py/Cargo.toml", "package"),
    ("./vec-utils-py/pyproject.toml", "project"),
]

for i in locations:
    with open(i[0], "r") as file:
        data = toml.load(file)
    data[i[1]]["version"] = VERSION
    # with open(i[0], "w") as file:
    #     toml.dump(data, file)
