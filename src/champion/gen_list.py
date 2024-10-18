import json

# Open and read the JSON file
with open("champion.json", "r") as file:
    data = json.load(file)

data = data["data"]

with open("list.rs", "w") as file:
    file.write("pub(crate) const LIST: [(u32, &str); " + str(len(data)) + "] = [\n")
    for champion in data:
        champion = data[champion]
        file.write("(" + champion["key"] + ',"' + champion["name"] + '"),' + "\n")
    file.write("];")

print(data)
