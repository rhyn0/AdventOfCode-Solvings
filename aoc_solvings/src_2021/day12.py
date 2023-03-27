"""Advent of Code 2021 Day12 problem.

Usage:
    day12.py [--example | --local] [--verbose]

Options:
    --example   Use example input rather than running personal input.
    --local     Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose   Use python logging to get verbose output of what is going on
                in a log file.
"""
from __future__ import annotations

# Standard Library
from collections import defaultdict
from collections import deque
import logging
import os
from pathlib import Path
import sys
from textwrap import dedent

# External Party
from aocd import get_data
from aocd import submit
from docopt import docopt

try:
    # My Modules
    from common.template import Day
except ImportError:
    sys.path.insert(0, os.path.dirname(sys.path[0]))
    # My Modules
    from common.template import Day

LOG_NAME = "day12"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end"""
)


class Day12(Day):
    """Day 12 of Advent of Code 2021."""

    def parse(self, puzzle_input: str) -> dict[str, set[str]]:
        """Return dictionary of connected caves for each cave."""
        return_d = defaultdict(set)
        for line in puzzle_input.splitlines():
            first, sec = line.split("-")
            return_d[first].add(sec)
            return_d[sec].add(first)
        return dict(return_d)

    @staticmethod
    def _is_small(cave_name: str) -> bool:
        return cave_name.lower() == cave_name

    @staticmethod
    def _update_caves(
        cave_dict: defaultdict[str, int], cave_update: str, *, _incr: int = 1
    ) -> tuple[int, defaultdict[str, int]]:
        caves_copy = cave_dict.copy()
        caves_copy[cave_update] += _incr
        return caves_copy[cave_update], caves_copy

    def traverse_paths(  # BFS approach will almost always look like this
        self, cave_conns: dict[str, set[str]], small_limit: int = 1
    ) -> int:
        """Generate all uniq paths from 'start' node to 'end' node.

        There is a limit to number of times to visit a small cave.
        If the limit is not 1, then only one small cave can have that exception.

        Args:
            cave_conns (dict[str, set[str]]): list of connections for a source cave
            small_limit (int, optional): limit on visiting small caves. Defaults to 1.

        Returns:
            int: Number of unique paths
        """
        num_paths = 0
        que: deque[tuple[str, defaultdict[str, int], bool]] = deque(
            [("start", defaultdict(int), False)]
        )
        while que:
            curr_cave, prev_seen, smalls_full = que.popleft()
            if curr_cave == "end":
                num_paths += 1
                continue
            for cave in cave_conns[curr_cave]:
                is_small_cave = self._is_small(cave)
                double_visit_one_small_cave = (
                    small_limit != 1 and smalls_full and prev_seen[cave] >= 1
                )
                if (
                    cave == "start"
                    or is_small_cave
                    and (prev_seen[cave] >= small_limit or double_visit_one_small_cave)
                ):
                    continue
                LOG.debug(
                    "At cave %r moving to %r and previously seen %r with limit of %d\
                        and it has visited a small cave twice %r",
                    curr_cave,
                    cave,
                    prev_seen,
                    small_limit,
                    smalls_full,
                )
                cave_seen_count, copy_prev_seen = self._update_caves(prev_seen, cave)
                curr_cave_double_visit = smalls_full or (
                    is_small_cave and cave_seen_count >= small_limit
                )
                que.append((cave, copy_prev_seen, curr_cave_double_visit))
        return num_paths

    def part1(self, data: dict[str, set[str]]) -> int:
        """Return number of distinct paths from 'start' to 'end'."""
        LOG.info("-" * 20 + "starting part1" + "-" * 20)
        return self.traverse_paths(data)

    def part2(self, data: dict[str, set[str]]) -> int:
        """Return value for the password that Cube wants."""
        LOG.info("-" * 20 + "starting part2" + "-" * 20)
        return self.traverse_paths(data, small_limit=2)


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 12, 2021
    day = Day12()

    if args["--example"] or args["--verbose"]:
        handle = logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.log", "w")
        handle.setFormatter(
            logging.Formatter("%(funcName)s-%(levelname)s:%(lineno)d %(message)s")
        )
        LOG.addHandler(handle)
        LOG.setLevel(logging.DEBUG)

    if args["--example"]:
        data = EXAMPLE
    elif args["--local"]:
        data = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        data = get_data(day=DAY, year=YEAR)
    answers = day.solve(data)
    print(answers)
    if args["--example"]:
        assert answers == (10, 36)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
