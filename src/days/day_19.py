import re

with open("../../inputs/input_19.txt", "r") as f:
    input = f.read()

molecule = input.split("\n")[-2][::-1]
reps = {m[1][::-1]: m[0][::-1] for m in re.findall(r"(\w+) => (\w+)", input)}


def rep(x):
    return reps[x.group()]


count = 0
while molecule != "e":
    print(molecule[::-1])
    molecule = re.sub("|".join(reps.keys()), rep, molecule, count=1)
    count += 1

print(count)
