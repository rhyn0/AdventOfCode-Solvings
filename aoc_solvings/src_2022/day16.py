"""Advent of Code Day16 problem.

Usage:
    day16.py [--example]

Options:
    --example   Use example input rather than running personal input.
"""
from __future__ import annotations

# Standard Library
from collections import defaultdict
from dataclasses import dataclass
from dataclasses import field
from itertools import product
import logging
import os
import re
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

LOG_NAME = "day16"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II"""
)


@dataclass
class Valve:
    """Dataclass for holding info about Valves."""

    name: str
    flow_rate: int
    connected: set[str] = field(default_factory=set)


class ValveSystem:
    """Necessary processing around Valves for this problem set."""

    def __init__(self, pipes: dict[str, Valve], time_budget: int) -> None:
        """Initialize all processing sets with necessary subsets of valves."""
        self.pipes = pipes
        self.flow_pipes = {
            name: valve for name, valve in pipes.items() if valve.flow_rate
        }
        self.open_pipes = set()
        self.minutes = time_budget
        self.curr_valve = "AA"
        self.steps = self._compute_steps()
        self.valve_history = defaultdict(int)
        self.states = {x: 1 << i for i, x in enumerate(self.flow_pipes)}

    def _compute_steps(self) -> dict[str, dict[str, float]]:
        dist_to_valves = {
            valve_name: {
                conn_valve_name: 1.0
                if conn_valve_name in self.pipes[valve_name].connected
                else float("inf")
                for conn_valve_name in self.pipes
            }
            for valve_name in self.pipes
        }
        for inter in dist_to_valves:
            for start in dist_to_valves:
                for end in dist_to_valves:
                    dist_to_valves[start][end] = min(
                        dist_to_valves[start][end],
                        dist_to_valves[start][inter] + dist_to_valves[inter][end],
                    )

        return dist_to_valves

    def dfs(
        self,
        valve_point: str,
        curr_state: int = 0,
        flow_rate: int = 0,
        time_budget: int | None = None,
    ) -> dict:
        """Visit valves with time budget and find flow values for visiting states."""
        if time_budget is None:
            time_budget = self.minutes
        LOG.debug(
            "At time remaining of %d, set history of state %s to %d",
            time_budget,
            curr_state,
            max(self.valve_history[curr_state], flow_rate),
        )
        self.valve_history[curr_state] = max(self.valve_history[curr_state], flow_rate)
        for valve in self.flow_pipes:
            new_time_budget = time_budget - int(self.steps[valve_point][valve]) - 1
            # if dest node is already visited by this state skip
            if curr_state & self.states[valve] or new_time_budget <= 0:
                continue
            LOG.debug(
                "Testing going to valve %r with new flow %d after move.",
                valve,
                flow_rate + self.flow_pipes[valve].flow_rate * new_time_budget,
            )
            self.dfs(
                valve,
                curr_state | self.states[valve],
                flow_rate + self.flow_pipes[valve].flow_rate * new_time_budget,
                new_time_budget,
            )

        return self.valve_history


class Day16(Day):
    """Day 16 of Advent of Code 2022."""

    def parse(self, puzzle_input: str) -> list[Valve]:
        """Return ValveSystem according to input system."""
        valves = []
        ptrrn = re.compile(r"^Valve ([A-Z]+)\D+=(\d+);[a-z\s]+([A-Z,\s]+)+")
        for line in puzzle_input.splitlines():
            LOG.debug(ptrrn.findall(line))
            valve_name, flow_rate, conn_valves = ptrrn.findall(line)[0]
            # here conn valves is the csv of connected ones
            valves.append(
                Valve(
                    valve_name,
                    int(flow_rate),
                    {name.strip() for name in conn_valves.split(",")},
                )
            )
        LOG.info("Got valves of %s", valves)
        return valves

    def part1(self, data: list[Valve]) -> int:
        """Find maximum pressure released acting alone in 30 minutes.

        Args:
            data (list[Valve]): List of all valves making up graph

        Returns:
            int
        """
        LOG.info("-" * 20 + "starting part1" + "-" * 20)
        vs = ValveSystem({valve.name: valve for valve in data}, time_budget=30)
        valve_flow_rates = vs.dfs("AA")
        LOG.debug("After visiting all spots, got state values of %s", valve_flow_rates)
        return max(valve_flow_rates.values())

    def part2(self, data: list[Valve]) -> int:
        """Return maximum pressure that can be released with 2 workers in 26 minutes.

        Args:
            data (list[Valve]): List of all valves in system

        Returns:
            int
        """
        LOG.info("-" * 20 + "starting part2" + "-" * 20)
        vs = ValveSystem({valve.name: valve for valve in data}, time_budget=26)
        valve_flow_rates = vs.dfs("AA")
        LOG.debug("After visiting all spots, got state values of %s", valve_flow_rates)
        return max(
            v1 + v2
            for (k1, v1), (k2, v2) in product(valve_flow_rates.items(), repeat=2)
            if not k1 & k2
        )


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 16, 2022
    day = Day16()
    if args["--example"]:
        handle = logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.txt", "w")
        handle.setFormatter(
            logging.Formatter("%(funcName)s-%(levelname)s:%(lineno)d %(message)s")
        )
        LOG.addHandler(handle)
        LOG.setLevel(logging.DEBUG)
        data = EXAMPLE
    else:
        data = get_data(day=DAY, year=YEAR)
    answers = day.solve(data)
    print(answers)
    if args["--example"]:
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
