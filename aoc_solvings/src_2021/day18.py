from __future__ import annotations

# Standard Library
import itertools
import math
import os
import re
import sys
from textwrap import dedent

try:
    # My Modules
    from common.log import get_logger
    from common.template import Day
    from common.template import main
except ImportError:
    sys.path.insert(0, os.path.dirname(sys.path[0]))
    # My Modules
    from common.log import get_logger
    from common.template import Day
    from common.template import main


LOG = get_logger("day18")


EXPLODE_DEPTH = 4
SPLIT_POINT = 10


def add_snail(left: str, right: str) -> str:
    """Return the snail number composed of left and right."""
    return f"[{left},{right}]"


def reduce_snail(snail: str) -> str:
    """Apply operations of explode and split to simplify the SnailNumber."""
    exploded = explode_snail(snail)
    if exploded != snail:
        return reduce_snail(exploded)

    split = split_snail(snail)
    if split != snail:
        return reduce_snail(split)
    # already at simplified form
    return snail


def split_snail(snail: str) -> str:
    """Split numbers that are greater than or equal to 10.

    Split numbers are replaced by a Snail pair representing the number

    Args:
        snail (str): SnailNumber

    Returns:
        str: Split version of snail number, if any
    """
    if (two_digit_match := re.search(r"\d{2}", snail)) is None:
        return snail
    LOG.debug("Found double digit number, splitting it - %r", two_digit_match)
    left_side = snail[: two_digit_match.start()]
    right_side = snail[two_digit_match.end() :]

    value = int(two_digit_match.group())
    left_digit = value // 2
    right_digit = math.ceil(value / 2)
    LOG.debug(
        "Split %s into %d and %d, respectively",
        two_digit_match.group(),
        left_digit,
        right_digit,
    )
    new_snail = f"{left_side}[{left_digit},{right_digit}]{right_side}"

    LOG.debug(
        "Split of %s turned the new snail into %r", two_digit_match.group(), new_snail
    )
    return new_snail


def explode_snail(snail: str) -> str:
    """Find nested snail numbers and explode them.

    Explosion adds the left number to the closest left number.
    Same for the right.
    Replaces the exploded number with a 0.
    """
    curr_idx = 0
    # has to match multi digit, explode before split
    LOG.debug("Working on %s", snail)
    while True:
        if (re_match := re.search(r"\[(\d+),(\d+)\]", snail[curr_idx:])) is None:
            break
        number_of_open_bracket = snail[: re_match.start() + curr_idx].count("[")
        number_of_close_bracket = snail[: re_match.start() + curr_idx].count("]")
        # we do up to the start of the match, has to be nested inside 4 other numbers
        if number_of_open_bracket - number_of_close_bracket < EXPLODE_DEPTH:
            curr_idx += re_match.end()
            LOG.debug(
                "Found match that wasn't correct depth (%d) - %s, moving offset to %d",
                number_of_open_bracket - number_of_close_bracket,
                re_match.group(),
                curr_idx,
            )
            continue
        LOG.debug("Found match %s after offset %d", re_match.groups(), curr_idx)
        left_num = re_match.group(1)
        right_num = re_match.group(2)

        # need to add left_num to the l
        # regex doesn't have a backwards search so reverseeft most number from it
        left_side = snail[: re_match.start() + curr_idx][::-1]
        right_side = snail[re_match.end() + curr_idx :]

        if next_left_number_match := re.search(r"\d+", left_side):
            LOG.debug(
                "closest left number is %s",
                next_left_number_match.string[
                    next_left_number_match.start() : next_left_number_match.end()
                ],
            )
            new_num = int(left_num) + int(
                left_side[
                    next_left_number_match.end()
                    - 1 : next_left_number_match.start()
                    - 1 : -1
                ]
            )
            # replace the number, remember to leave the number backwards
            left_side = (
                left_side[: next_left_number_match.start()]
                + str(new_num)[::-1]
                + left_side[next_left_number_match.end() :]
            )

        if next_right_number_match := re.search(r"\d+", right_side):
            new_num = int(right_num) + int(
                right_side[
                    next_right_number_match.start() : next_right_number_match.end()
                ]
            )
            right_side = (
                right_side[: next_right_number_match.start()]
                + str(new_num)
                + right_side[next_right_number_match.end() :]
            )

        snail = left_side[::-1] + "0" + right_side
        LOG.debug("Created new %r", snail)
        return snail
    return snail


def magnitude_snail(snail: str) -> int:
    """Return the magnitude of a Snail Number.

    Magnitude is recursively, 3 times the left plus 2 times the right.

    Args:
        snail (str): Snail Number

    Returns:
        int: Magnitude of the number

    Examples:
        >>> magnitude_snail("[[1,2],[[3,4],5]]")
        143
        >>> magnitude_snail("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        1384
        >>> magnitude_snail("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        445
        >>> magnitude_snail("[[[[3,0],[5,3]],[4,4]],[5,5]]")
        791
        >>> magnitude_snail("[[[[5,0],[7,4]],[5,5]],[6,6]]")
        1137
        >>> magnitude_snail("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        3488
    """

    def magnitude_calc(left: int, right: int) -> int:
        return 3 * left + 2 * right

    int_pair_snail = re.compile(r"\[(\d+),(\d+)\]")
    while snail.count(",") > 1:
        if (pair_match := int_pair_snail.search(snail)) is None:
            break
        LOG.debug("Found match %s", pair_match.groups())

        left_num = int(pair_match.group(1))
        right_num = int(pair_match.group(2))

        # replace this int pair with the magnitude of the pair
        magnitude = magnitude_calc(left_num, right_num)
        snail = snail[: pair_match.start()] + str(magnitude) + snail[pair_match.end() :]
        LOG.debug("Updated snail is now %r", snail)
    # now we are a singular pair [x,y]
    left, right = map(int, snail[1:-1].split(","))
    return magnitude_calc(left, right)


class Day18(Day):
    """Day 17 of Advent of Code 2021."""

    example = dedent(
        """\
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"""
    )
    day = 18
    year = 2021

    def parse(self, puzzle_input: str) -> list[str]:
        """Return bin string with no extra spaces."""
        return puzzle_input.splitlines()

    def part1(self, data: list[str]) -> int:
        """Return sum of the packet versions for all packets in data."""
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        result = None
        for num in data:
            print("=" * 80)
            curr = add_snail(result, num) if result is not None else num  # type: ignore[arg-type]
            print(curr)
            result = reduce_snail(curr)
            print(result)
        if result is None:
            raise TypeError("input data should be non empty list")  # noqa: TRY003
        return magnitude_snail(result)

    def part2(self, data: list[str]) -> int:
        """Return count of max element minus count of min element after insertions."""
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        return max(
            magnitude_snail(reduce_snail(add_snail(*pair)))
            for pair in itertools.permutations(data, 2)
        )


if __name__ == "__main__":
    main(Day18())
