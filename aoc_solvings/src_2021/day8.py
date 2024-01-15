# Standard Library
from collections import namedtuple
import re
from typing import ClassVar
from typing import NamedTuple

# External Party
from aocd import get_data
from aocd import submit

# My Modules
from common.template import Day

Notes = namedtuple("Notes", ["inputs", "outputs"])


class Day8(Day):
    """Day 8 of Advent of Code 2021.

    Work with 7 segment displays, solve which input causes what segment.

    8: numbered the segments out
      0000
     1    2
     1    2
      3333
     4    5
     4    5
      6666
    """

    def parse(self, puzzle_input: str) -> list[NamedTuple]:
        """Return notes for given input, split on |."""
        return [
            Notes(
                [val.strip() for val in line.split("|")[0].split()],
                [val.strip() for val in line.split("|")[1].split()],
            )
            for line in puzzle_input.split("\n")
        ]

    def part1(self, data: list[Notes]) -> int:
        """Return number of 1, 4, 7, 8 in the output of the entries.

        Since 1, 4, 7, and 8 have unique number of used segments in
        a 7-segment display, easy to identify them

        Parameters
        ----------
        data : List[Notes]
            Each Notes has 10 unique input entries and 4 output entries

        Returns:
        -------
        int
            number of occurrences of 1,4,7,8 in the outputs

        """
        return sum(sum(len(val) in [2, 4, 3, 7] for val in x.outputs) for x in data)

    # unique lengths: num_seg -> number
    #     2 -> 1
    #     3 -> 7
    #     4 -> 4
    #     7 -> 8

    # contested lengths:
    #     5 -> 2, 3, 5
    #     6 -> 0, 6, 9

    num_to_seg: ClassVar = {
        "0": [0, 1, 2, 4, 5, 6],
        "1": [2, 5],
        "2": [0, 2, 3, 4, 6],
        "3": [0, 2, 3, 5, 6],
        "4": [1, 2, 3, 5],
        "5": [0, 1, 3, 5, 6],
        "6": [0, 1, 3, 4, 5, 6],
        "7": [0, 2, 5],
        "8": [0, 1, 2, 3, 4, 5, 6],
        "9": [0, 1, 2, 3, 5, 6],
    }

    @staticmethod
    def craft_num(segments: dict, entry: str) -> str:
        """Given inputs, output which number that segment set is."""
        entry_list = sorted(segments[c] for c in entry)
        for i, val in Day8.num_to_seg.items():
            if val == entry_list:
                return i
        raise IndexError

    def segment_to_num(self, segments: dict, note: Notes) -> int:
        """Return integer that note outputs."""
        return int("".join(self.craft_num(segments, entry) for entry in note.outputs))

    def part2(self, data: list[Notes]) -> int:
        """Do part 2."""

        def strip(string: str, *stripping) -> str:
            stripped_together = "".join(stripping)
            return re.sub(f"[{stripped_together}]", "", string)

        solved_outputs: list[int] = []
        for note in data:
            # set the dictionary
            num_seg: dict[int, list[str]] = {2: [], 3: [], 4: [], 5: [], 6: [], 7: []}
            for key in num_seg:
                num_seg[key].extend([seg for seg in note.inputs if len(seg) == key])
            # 2 subtract 4 segs leaves 3 segs
            two = next(val for val in num_seg[5] if len(strip(val, num_seg[4][0])) == 3)
            three = next(  # 3 subtract 1 segs leaves 3 segments
                val for val in num_seg[5] if len(strip(val, num_seg[2][0])) == 3
            )
            # remove a 1 from a 4 then remove those segs from a 5 to get only 3
            five = next(
                val
                for val in num_seg[5]
                if len(strip(val, strip(num_seg[4][0], num_seg[2][0]))) == 3
            )
            segments = {}
            segments[num_seg[3][0].strip(num_seg[2][0])] = 0
            segments[
                strip(
                    five,
                    three,
                )
            ] = 1
            segments[strip(num_seg[4][0], five)] = 2
            segments[strip(two, three)] = 4
            segments[strip(five, num_seg[4][0], num_seg[3][0])] = 6
            segments[
                strip(
                    three,
                    num_seg[3][0].strip(num_seg[2][0]),
                    strip(
                        five,
                        num_seg[4][0],
                        num_seg[3][0],
                    ),
                    num_seg[2][0],
                )
            ] = 3
            segments[
                strip(
                    five,
                    num_seg[3][0].strip(num_seg[2][0]),
                    strip(
                        five,
                        num_seg[4][0],
                        num_seg[3][0],
                    ),
                    strip(num_seg[4][0], num_seg[2][0]),
                )
            ] = 5
            solved_outputs.append(self.segment_to_num(segments=segments, note=note))
        return sum(solved_outputs)


if __name__ == "__main__":
    day = Day8()
    answers = day.solve(get_data(day=8, year=2021))
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, part=part, day=8, year=2021)
