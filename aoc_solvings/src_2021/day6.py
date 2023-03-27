# Standard Library

# External Party
from aocd import get_data
from aocd import submit

# My Modules
from common.template import Day


class Day6(Day):
    """Day6 of Advent of Code 2021."""

    def parse(self, puzzle_input: str) -> list[int]:
        """Given list of fish states i think - return int version."""
        return [int(val) for val in puzzle_input.split(",")]

    def part1(self, data: list[int]) -> int:
        """Not really sure as I'm writing this later.

        Should be something about simulating fish spawn for a given number of days.
        """
        sim_days = 80
        fishes = data.copy()
        while sim_days > 0:
            babies = 0
            for i, val in enumerate(fishes):
                if val == 0:
                    babies += 1
                    fishes[i] = 6
                else:
                    fishes[i] -= 1
            fishes.extend([8 for _ in range(babies)])
            sim_days -= 1
        return len(fishes)

    def part2(self, data: list[int]) -> int:
        """Not really sure as I'm writing this later.

        Should be something about simulating fish spawn for a given number of days.
        Then counting them by what state they are in, since they spawn
        on a 10 day count or something
        """
        current_states = {
            0: data.count(0),
            1: data.count(1),
            2: data.count(2),
            3: data.count(3),
            4: data.count(4),
            5: data.count(5),
            6: data.count(6),
            7: data.count(7),
            8: data.count(8),
        }
        for _ in range(256):
            next_states = {
                0: current_states[1],
                1: current_states[2],
                2: current_states[3],
                3: current_states[4],
                4: current_states[5],
                5: current_states[6],
                6: current_states[7],
                7: current_states[8],
                8: current_states[0],
            }
            if current_states[0]:
                next_states[6] += current_states[0]
            current_states = next_states
        return sum(current_states.values())


if __name__ == "__main__":
    problem_day = Day6()
    answers = problem_day.solve(get_data(day=6, year=2021))
    print(answers[1])
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, part=part, day=6, year=2021)
