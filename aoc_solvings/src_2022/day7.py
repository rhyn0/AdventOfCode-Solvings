"""Advent of Code Day7 problem.

Usage:
    day7.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

Options:
    --example       Use example input rather than running personal input.
    --local         Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose       Use python logging to get verbose output of what is going
                    on in a log file.
    --quiet         Disable logging for example mode.
    --parts PART    Do only specified part, options are 'a', 'b', or 'ab'. [default: ab]
"""

# Standard Library
from collections import defaultdict
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

LOG_NAME = "day7"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k"""
)


class Day7(Day):
    """Day 7 of Advent of Code 2022."""

    def parse(self, data_input: str) -> list[str]:
        """Given input return it."""
        return data_input.split("\n")

    @staticmethod
    def _change_dir(command: list[str], path_stack: list[str]) -> list[str]:
        if command[2] == "..":
            return path_stack[:-1]
        return [*path_stack, command[2]]

    @staticmethod
    def _add_dir_size(
        new_size: int, curr_path: list[str], directory_sizes: defaultdict[str, int]
    ) -> defaultdict[str, int]:
        for i in range(len(curr_path) + 1):
            directory_sizes["/".join(curr_path[:i])] += new_size

        return directory_sizes

    def compute_filesystem(self, data_in: list[str]) -> dict[str, int]:
        """Return directory sizes for the discovered file system."""
        paths: defaultdict[str, int] = defaultdict(int)
        curr_path: list[str] = []
        for line in data_in:
            words = line.strip().split(" ")
            if words[1] == "cd":
                curr_path = self._change_dir(words, curr_path)
            elif words[0].isnumeric():
                paths = self._add_dir_size(int(words[0]), curr_path, paths)

        return paths

    def part1(self, data: list[str], max_dir_size: int = 100_000) -> int:
        """Find sum of directories that are smaller than 100_000.

        File system is explored in a terminal as input

        Args:
            data (List[str]): terminal output for traversing the directory
            max_dir_size (int): Filter for found directories basedd on size.

        Returns:
            int
        """
        return sum(
            val for val in self.compute_filesystem(data).values() if val <= max_dir_size
        )

    def part2(self, data: list[str]) -> int:
        """Find smallest directory to clear up space for update.

        Total system space is 70 mil, need to have 30 mil space for the update.
        Return the size of the directory that is the smallest delete possible
        to get to that free space.

        Args:
            data (List[str]): Commands of exploring FS

        Returns:
            int
        """
        max_usable_space = 70_000_000 - 30_000_000
        fs = self._compute_filesystem(data)
        need_to_clear = fs["/"] - max_usable_space
        smallest_dir = float("inf")
        for val in fs.values():
            if val >= need_to_clear:
                smallest_dir = min(smallest_dir, val)
        return smallest_dir if not isinstance(smallest_dir, float) else -1


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 7, 2022
    day = Day7()
    if args["--example"] or args["--verbose"]:
        handle = logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.log", "w")
        handle.setFormatter(
            logging.Formatter("%(funcName)s-%(levelname)s:%(lineno)d %(message)s")
        )
        LOG.addHandler(handle)
        LOG.setLevel(logging.DEBUG)
    if args["--quiet"]:
        logging.disable(logging.CRITICAL)

    if args["--example"]:
        grid = EXAMPLE
    elif args["--local"]:
        grid = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        grid = get_data(day=DAY, year=YEAR)
    answers = day.solve(grid, parts=args["--parts"][0])
    print(answers)
    if args["--example"]:
        assert answers == (95437, 24933642)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
