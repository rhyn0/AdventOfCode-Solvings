"""Advent of Code Day13 problem.

Usage:
    day13.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

Options:
    --example       Use example input rather than running personal input.
    --local         Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose       Use python logging to get verbose output of what is going on
                    in a log file.
    --quiet         Disable logging for example mode.
    --parts PART    Do only specified part, options are 'a', 'b', or 'ab'. [default: ab]
"""
# Standard Library
from collections.abc import Callable
from functools import cmp_to_key
from functools import wraps
from itertools import zip_longest
import json
import logging
import os
from pathlib import Path
import sys
from textwrap import dedent
from typing import Literal

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

LOG_NAME = "day13"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    [1,1,3,1,1]
    [1,1,5,1,1]

    [[1],[2,3,4]]
    [[1],4]

    [9]
    [[8,7,6]]

    [[4,4],4,4]
    [[4,4],4,4,4]

    [7,7,7,7]
    [7,7,7]

    []
    [3]

    [[[]]]
    [[]]

    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]"""
)


class Day13(Day):
    """Day 13 of Advent of Code 2022."""

    def parse(self, data_in: str) -> list[str]:
        """Return packets as a list of string contents."""
        return [line for line in data_in.splitlines() if line]

    @staticmethod
    def comparator_normalize(
        func: Callable[..., int]
    ) -> Callable[..., Literal[-1, 0, 1]]:
        """Normalize the returns to match a comparator.

        In other languages, comparators will return -1, 0, or 1 depending on if
        item comes before, matches, or after the compared to item.

        Args:
            func (Callable[..., int]): Function to normalize int return

        Returns:
            Callable[..., Literal[-1, 0, 1]]: Wrapped function
        """

        @wraps(func)
        def inner(*args, **kwargs) -> Literal[-1, 0, 1]:
            received_val = func(*args, **kwargs)
            if received_val < 0:
                return -1
            if received_val > 0:
                return 1
            return 0

        return inner

    def recurse_subpacket(
        self, left_pkt: list | int, right_pkt: list | int
    ) -> Literal[-1, 0, 1]:
        """Compare subpackets of differing types.

        Args:
            left_pkt (Union[List, int]): Either a subpacket or int
            right_pkt (Union[List, int]): Either a subpacket or int

        Returns:
            Literal[-1, 0, 1]: comparator normalized value
        """
        if not isinstance(left_pkt, list):
            left_pkt = [left_pkt]
        if not isinstance(right_pkt, list):
            right_pkt = [right_pkt]
        result = self._valid_packet(left_pkt, right_pkt)
        LOG.debug(
            "Given item %r, %r returning %d",
            left_pkt,
            right_pkt,
            result,
        )
        return result

    @comparator_normalize
    def _valid_packet(  # 3 distinct cases to consider at each subpacket item
        self,
        parsed_1: list[list[int] | int],
        parsed_2: list[list[int] | int],
    ) -> int:
        """Return the value of comparison between two packets.

        A packet is a list of lists and ints.

        Args:
            parsed_1 (List[Union[List[int], int]]): packet 1
            parsed_2 (List[Union[List[int], int]]): packet 2

        Returns:
            int: -1, when parsed_2 is greater, 0 when equal, 1 when parsed_1 is greater
        """
        sentinel = object()

        for item1, item2 in zip_longest(parsed_1, parsed_2, fillvalue=sentinel):
            LOG.debug(
                "Comparing parts of packet that is left: %r and right: %r", item1, item2
            )
            if sentinel in (item1, item2):  # readability
                LOG.debug("Given item %r, %r, one subpacket is too short", item1, item2)
                return -1 if item1 is sentinel else 1
            if isinstance(item1, int) and isinstance(item2, int) and item1 != item2:
                return item1 - item2
            if (isinstance(item1, list) or isinstance(item2, list)) and (
                result := self.recurse_subpacket(item1, item2)
            ) != 0:
                # mismatch type or lists
                return result
        # return nothing since this packet has no result so far
        LOG.debug(
            "Given item %r, %r non deterministic at this point", parsed_1, parsed_2
        )
        return 0

    def part1(self, data: list[str]) -> int:
        """Return sum of indices of valid packet pairs.

        Packets are 1-indexed and pairs do not overlap.
        Accidentally left a bug in here when refactoring for part2.

        Args:
            data (List[str]): List of packets in order

        Returns:
            int
        """
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        LOG.debug("input is %r", data)
        index_sum = 0
        for index, (first, second) in enumerate(
            zip(data[::2], data[1::2], strict=False), start=1
        ):
            LOG.debug("At index %d comparing %r to %r", index, first, second)
            if self._valid_packet(json.loads(first), json.loads(second)) == -1:
                LOG.debug("%r paired with %r is a valid order", first, second)
                index_sum += index

        return index_sum

    def part2(self, data: list[str]) -> int:
        """Product of divider indices after sorting packets.

        Packets are 1-indexed. Sort the packets into order then find the
        divider packets locations. Divider packets are [[2]], [[6]]

        Args:
            data (List[str]): List of packets in string contents

        Returns:
            int
        """
        dividers = [[[2]], [[6]]]
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        local_data = [json.loads(line) for line in data] + dividers
        LOG.debug("unsorted data is %s", "\n".join(str(line) for line in local_data))
        local_data.sort(key=cmp_to_key(self._valid_packet))
        LOG.debug("sorted data is %s", "\n".join(str(line) for line in local_data))
        return (1 + local_data.index(dividers[0])) * (1 + local_data.index(dividers[1]))


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 13, 2022
    day = Day13()
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
        data = EXAMPLE
    elif args["--local"]:
        data = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        data = get_data(day=DAY, year=YEAR)
    answers = day.solve(data, parts=args["--parts"][0])
    print(answers)
    if args["--example"]:
        assert answers == (13, 140)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
