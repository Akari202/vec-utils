VERSION = "0.2.5"

locations = [
    "./vec-utils/Cargo.toml",
    "./vec-utils-py/Cargo.toml",
    "./vec-utils-py/pyproject.toml",
]

for i in locations:
    with open(i, "r") as file:
        data = file.readlines()
    data[2] = f'version = "{VERSION}"\n'
    # for i in data:
    #     print(i, end="")
    # Avoid writing a CLRF
    # https://stackoverflow.com/questions/76382887/how-to-fix-the-line-ending-style-either-crlf-or-lf-in-python-when-written-a-te
    with open(i, "w", encoding="utf-8", newline="\n") as file:
        file.writelines(data)
    print(f"{i} version updated")
