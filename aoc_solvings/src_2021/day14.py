"""Advent of Code 2021 Day14 problem.

Usage:
    day14.py [--example [--quiet] | --local] [--verbose]

Options:
    --example   Use example input rather than running personal input.
    --local     Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose   Use python logging to get verbose output of what is going on
                in a log file.
    --quiet     Disable logging for example mode.
"""
from __future__ import annotations

# Standard Library
from collections import Counter
from collections import defaultdict
from dataclasses import dataclass
from itertools import pairwise
import logging
import os
from pathlib import Path
import sys
from textwrap import dedent
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Iterator

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

LOG_NAME = "day14"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C"""
)


@dataclass(unsafe_hash=True)
class PolymerRule:
    """Mapping of polymer pair to the inserted character."""

    poly_pair: str
    result_el: str

    @property
    def insertion(self) -> tuple[str, str]:
        """Get the pairs that result from having this pair occur in polymer template."""
        return (
            f"{self.poly_pair[0]}{self.result_el}",
            f"{self.result_el}{self.poly_pair[1]}",
        )


class PolymerRules:
    """Collection of PolymerRule."""

    def __init__(self) -> None:
        """Initialize with no rules."""
        self.rules: dict[str, PolymerRule] = {}

    def __iter__(self) -> Iterator[PolymerRule]:
        """Iterate over rules."""
        yield from self.rules.values()

    def __getitem__(self, ind: str) -> PolymerRule:
        """Get a rule based on the polymer pair string."""
        return self.rules[ind]

    def add_rule(self, rule: PolymerRule) -> None:
        """Add a rule to the collection."""
        self.rules[rule.poly_pair] = rule

    def process_template_pair(self, pair: str | tuple[str, str]) -> tuple[str, str]:
        """Get resulting pairs based on processing a polymer pair."""
        if isinstance(pair, tuple):
            pair = "".join(pair)
        return self.rules[pair].insertion


class Day14(Day):
    """Day 14 of Advent of Code 2021."""

    def parse(self, puzzle_input: str) -> tuple[str, PolymerRules]:
        """Return tuple of starting template and polymer rules."""
        start_templ, rules_list = puzzle_input.split("\n\n")
        rules_list = rules_list.splitlines()
        ruleset = PolymerRules()
        for rule in rules_list:
            ruleset.add_rule(PolymerRule(*(part.strip() for part in rule.split("->"))))

        return start_templ, ruleset

    @staticmethod
    def _polymer_insert_round(
        pair_counts: dict[str, int], ltr_counts: Counter[str], rules: PolymerRules
    ) -> tuple[dict[str, int], Counter[str]]:
        new_pairs = defaultdict(int)
        new_pairs.update(pair_counts)
        for rule in rules:
            primary_pair = rule.poly_pair
            inserted = rule.result_el
            left_pair, right_pair = rule.insertion

            if primary_pair not in pair_counts or pair_counts[primary_pair] <= 0:
                continue

            exist_count = pair_counts[primary_pair]

            ltr_counts[inserted] += exist_count
            new_pairs[primary_pair] -= exist_count
            new_pairs[left_pair] += exist_count
            new_pairs[right_pair] += exist_count

        return dict(new_pairs), ltr_counts

    def handle_polymer_rounds(
        self, template: str, rules: PolymerRules, /, rounds: int = 10
    ) -> Counter[str]:
        """Return a Counter of the letters in polymer after `rounds` rounds."""
        pair_dict = dict(Counter(["".join(pair) for pair in pairwise(template)]))
        letter_count = Counter(template)
        LOG.debug("Starting with pairs %s and %s counts", pair_dict, letter_count)
        for poly_round in range(rounds):
            pair_dict, letter_count = self._polymer_insert_round(
                pair_dict, letter_count, rules
            )
            LOG.debug(
                "After round %d, counts are %r",
                poly_round,
                letter_count,
            )

        LOG.info(
            "After running %d rounds, counts are %s",
            rounds,
            letter_count.most_common(),
        )
        return letter_count

    def part1(self, data: tuple[str, PolymerRules]) -> int:
        """Return count of max element minus count of min element after insertions."""
        LOG.info("-" * 20 + "starting part1" + "-" * 20)
        counts = self.handle_polymer_rounds(*data)
        return counts.most_common()[0][1] - counts.most_common()[-1][1]

    def part2(self, data: tuple[str, PolymerRules]) -> int:
        """Return count of max element minus count of min element after insertions."""
        LOG.info("-" * 20 + "starting part2" + "-" * 20)
        counts = self.handle_polymer_rounds(*data, rounds=40)
        return counts.most_common()[0][1] - counts.most_common()[-1][1]


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 14, 2021
    day = Day14()
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
    answers = day.solve(data)
    print(answers)
    if args["--example"]:
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
