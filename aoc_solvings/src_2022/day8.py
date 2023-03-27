# Standard Library
from itertools import pairwise
import os
import sys

# External Party
from aocd import get_data
from aocd import submit

try:
    # My Modules
    from common.template import Day
except ImportError:
    sys.path.insert(0, os.path.dirname(sys.path[0]))
    # My Modules
    from common.template import Day


class Day8(Day):
    """Day 8 of Advent of Code 2022."""

    _dirs = (0, 1, 0, -1, 0)

    def parse(self, data_input: str) -> list[list[int]]:
        """Given input return it."""
        return [[int(char) for char in line] for line in data_input.split("\n")]

    def _traverse_r(self, row: int, data: list[list[int]]) -> set[tuple[int, int]]:
        begin, end = 0, len(data[row]) - 1
        begin_prev, end_prev = -1, -1
        ret_set = set()
        while begin < end:
            if data[row][begin] > begin_prev:
                ret_set.add((row, begin))
                begin_prev = data[row][begin]
            if data[row][end] > end_prev:
                ret_set.add((row, end))
                end_prev = data[row][end]

            if data[row][begin] >= data[row][end]:
                end -= 1
            else:
                begin += 1

        return ret_set

    def _traverse_c(self, col: int, data: list[list[int]]) -> set[tuple[int, int]]:
        begin, end = 0, len(data) - 1
        begin_prev, end_prev = -1, -1
        ret_set = set()
        while begin < end:
            if data[begin][col] > begin_prev:
                ret_set.add((begin, col))
                begin_prev = data[begin][col]
            if data[end][col] > end_prev:
                ret_set.add((end, col))
                end_prev = data[end][col]

            if data[begin][col] >= data[end][col]:
                end -= 1
            else:
                begin += 1

        return ret_set

    def part1(self, data: list[list[int]]) -> int:
        """Find the number of visible trees from outside forest.

        Tree is only visible from outside if it's not blocked by a
        same height or taller tree.

        Args:
            data (List[List[int]]): Grid of tree heights, ranging 0-9

        Returns:
            int
        """
        # two pointer approach, move the smaller tree pointer
        # or left one if equal until same index
        n, m = len(data), len(data[0])
        tree_visible = set()

        for row_no in range(n):
            if row_no == 0 or row_no == n - 1:
                tree_visible.update({(row_no, col) for col in range(m)})
            else:
                tree_visible.update(self._traverse_r(row_no, data))

        for col_no in range(m):
            if col_no == 0 or col_no == m - 1:
                tree_visible.update({(row, col_no) for row in range(n)})
            else:
                tree_visible.update(self._traverse_c(col_no, data))

        # print(*tree_visible, sep="\n")
        return len(tree_visible)

    def part2(self, data: list[list[int]]) -> int:
        """Return max scenic score for a tree in forest.

        Scenic score is product of number of trees visible in cardinal
        directions from that spot.
        Trees follow `part1` visibility rules.
        Since trees on the outside have a direction where they can see 0 trees,
        we can ignore those.

        Args:
            data (List[List[int]]): Grid of tree heights, ranging 0-9

        Returns:
            int
        """
        n, m = len(data), len(data[0])
        ans = 0
        for row in range(n):
            for col in range(m):
                score = 1
                for dr, dc in pairwise(self._dirs):
                    row_curr = row + dr
                    col_curr = col + dc
                    dist = 0
                    while (
                        0 <= row_curr < n
                        and 0 <= col_curr < m
                        and data[row][col] > data[row_curr][col_curr]
                    ):
                        dist += 1
                        row_curr += dr
                        col_curr += dc
                        if (
                            0 <= row_curr < n
                            and 0 <= col_curr < m
                            and data[row][col] <= data[row_curr][col_curr]
                        ):
                            # stopping at higher tree inside grid is a plus one
                            dist += 1
                    score *= dist
                ans = max(ans, score)

        return ans


if __name__ == "__main__":
    DAY, YEAR = 8, 2022
    day = eval(f"Day{DAY}()")
    answers = day.solve(get_data(day=DAY, year=YEAR))
    # print(answers)
    for ans, part in zip(answers, ["a", "b"], strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
