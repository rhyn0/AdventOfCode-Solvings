"""Day 1 of Advent of Code 2021."""


def part1(values):
    """Count times there is a new maximum in path."""
    prev = values[0]
    increase_count = 0
    for val in values[1:]:
        if val > prev:
            increase_count += 1
        prev = val
    return increase_count


def part2(values):
    """Find new maximums for points summed over 3."""
    windows = [sum(values[i : i + 3]) for i in range(len(values) - 1)]
    return part1(windows)


if __name__ == "__main__":
    with open("data/input1.txt") as f:
        values = [int(x) for x in f.readlines()]
    print(part2(values))
