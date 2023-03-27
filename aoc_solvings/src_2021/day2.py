"""Day 2 of Advent of Code 2021."""


def part1(data):
    """Return product of horizontal and depth after submarine moves.

    Args:
        data: Instructions on moving the sub

    Returns:
        int: product of horizontal and vertical change
    """
    horiz, depth = 0, 0
    for value in data:
        value = value.split(" ")
        match value[0]:
            case "forward":
                horiz += int(value[1])
            case "down":
                depth += int(value[1])
            case "up":
                depth -= int(value[1])
            case _:
                print(value, "this thing doesn't work!!!")
                return  # noqa: R502
    return horiz * depth


def part2(data):
    """Change piloting style to be aerial, nose-heading."""
    horiz, depth, aim = 0, 0, 0
    for value in data:
        value = value.split(" ")
        match value[0]:
            case "forward":
                horiz += int(value[1])
                depth += aim * int(value[1])
            case "down":
                aim += int(value[1])
            case "up":
                aim -= int(value[1])
            case _:
                print(value, "this thing doesn't work!!!")
                return  # noqa: R502
    return horiz * depth


if __name__ == "__main__":
    with open("data/input2.txt") as f:
        values = f.readlines()
    print(part2(values))
