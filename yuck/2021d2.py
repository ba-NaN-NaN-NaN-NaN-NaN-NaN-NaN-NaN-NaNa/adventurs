#!/usr/bin/env python3

f = open("../input/2021_d2.txt")
lines = f.readlines()
f.close()

sample = """
forward 5
down 5
forward 8
up 3
down 8
forward 2
""".split("\n")

sample = [l.strip() for l in lines if len(l.strip()) > 0]

lines = sample
print(lines)


depth = 0
horiz_pos = 0
aim = 0



for line in lines:
    parts = line.split(" ")
    print(parts)
    direction = parts[0]
    metres = int(parts[1])

    if direction == "forward":
        horiz_pos += metres
        depth += aim * metres

    elif direction == "ddcforward":
        horiz_pos += metres

    elif direction == "down":
        #depth += metres
        aim += metres

    elif direction == "up":
        #depth -= metres
        aim -= metres

    else:
        print("??  %s" % direction)


print("Part 1 -> %d * %d = %d" % (depth, horiz_pos, depth * horiz_pos))
