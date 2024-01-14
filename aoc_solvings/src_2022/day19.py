"""Advent of Code Day19 problem.

Usage:
    day19.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

Options:
    --example       Use example input rather than running personal input.
    --local         Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose       Use python logging to get verbose output of what is going on
                    in a log file.
    --quiet         Disable logging for example mode.
    --parts PART    Do only specified part, options are 'a', 'b', or 'ab'. [default: ab]
"""
from __future__ import annotations

# Standard Library
from collections import defaultdict
from collections import deque
from dataclasses import dataclass
from enum import Enum
import logging
from math import prod
import os
from pathlib import Path
import re
import sys
from textwrap import dedent
from typing import TYPE_CHECKING
from typing import TypeAlias

if TYPE_CHECKING:
    from collections.abc import Iterable

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

LOG_NAME = "day19"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."""
)


class Robot(Enum):
    """Classes of materials and robots necessary."""

    ORE = 1
    CLAY = 2
    OBSIDIAN = 3
    GEODE = 4


Resource = Robot


@dataclass
class FactoryBlueprint:
    """Holds blueprint for building in robot factory."""

    bp_id: int
    robot_costs: dict[Robot, dict[Resource, int]]

    def makeable_bots(self, curr_resources: dict[Resource, int]) -> set[Robot]:
        """Return which possible bots are creatable given resources.

        If geode robot is possible to make, only return that since creating
        it earlier is always good option.
        """
        options = {
            robt
            for robt, cost_dict in self.robot_costs.items()
            if all(quant <= curr_resources[res] for res, quant in cost_dict.items())
        }
        options.add(0)  # type: ignore
        if Robot.GEODE in options:
            return {Robot.GEODE}
        return options

    def robot_count_max(self) -> dict[Robot, int]:
        """For each resource need the maximum resource requirement of robots at a time.

        Example:
            if obisidian robots cost 7 ore, which is greater than any other ore cost
            there is no benefit in having more than 7 ore robots
        """
        robot_max_counts = {
            robot: max(cost.get(robot, 0) for cost in self.robot_costs.values())
            for robot in Robot
        }
        # arbitrarily large number because there is no limit for our prize
        robot_max_counts[Robot.GEODE] = 300
        return robot_max_counts

    def build_bot(
        self, robots: dict[Robot, int], resources: dict[Resource, int], building: Robot
    ) -> tuple[dict[Robot, int], dict[Resource, int]]:
        """Return new robot and resource dict after building based on option."""
        cost = self.robot_costs[building]
        new_robots = {
            robt: count + (1 if building == robt else 0)
            for robt, count in robots.items()
        }
        local_resources = resources.copy()
        for res, res_cost in cost.items():
            local_resources[res] -= res_cost

        return new_robots, local_resources


TimedQueueSearchItem: TypeAlias = tuple[
    int, dict[Robot, int], dict[Resource, int], set[Robot]
]
QueueSearchItem: TypeAlias = tuple[dict[Robot, int], dict[Resource, int], set[Robot]]


class Satchel:
    """Object to hold the minutes for a run and do math."""

    def __init__(self, minutes: int) -> None:
        """Hold the max minutes for a calculation."""
        self.max_minutes = minutes

    @staticmethod
    def increase_resources(
        robots: dict[Robot, int], exist_resources: dict[Resource, int]
    ) -> dict[Resource, int]:
        """Let robots increase the resource count at end of minute."""
        return {res: robots[res] + exist_resources[res] for res in Resource}

    @staticmethod
    def starting_actors_set() -> tuple[dict[Resource, int], dict[Robot, int]]:
        """Create objects that represent the beginning of a Blueprint session.

        No resources, and only one ORE Robot.
        """
        resources = {res: 0 for res in Resource}
        robots = {robt: 0 for robt in Robot}
        robots[Robot.ORE] = 1
        return resources, robots

    def search_build_options(
        self,
        options: set[Robot],
        curr_res: dict[Resource, int],
        curr_bots: dict[Robot, int],
        skipped: set[Robot],
    ) -> Iterable[QueueSearchItem]:
        """Return Iterable of all paths possible from this decision point.

        From given set of resources/robots, return possible/desirable build options.

        Args:
            options (set[Robot]): Possible Robots that can be built
            curr_res (dict[Resource, int]): Current resource count
            curr_bots (dict[Robot, int]): Current robot count
            skipped (set[Robot]): Actions that were skipped last decision

        Returns:
            Iterable[QueueSearchItem]: Iterable object of the decisions
        """
        builds: list[QueueSearchItem] = []
        for build_option in options:
            if not build_option:
                builds.append(
                    (
                        curr_bots.copy(),
                        self.increase_resources(curr_bots, curr_res),
                        options,
                    )
                )
            elif (
                build_option not in skipped
                and curr_bots[build_option] + 1 <= self.max_robots[build_option]
            ):
                bots, resources = self.blueprint.build_bot(
                    curr_bots, curr_res, build_option
                )
                builds.append(
                    (
                        bots,
                        self.increase_resources(curr_bots, resources),
                        set(),
                    )
                )
        return builds

    def set_blueprint(self, bp: FactoryBlueprint) -> None:
        """Set the current running blueprint.

        Also calculates the Robot maximum for this blueprint.
        """
        self.blueprint = bp
        self.max_robots = self.blueprint.robot_count_max()

    def blueprint_score(self, bp: FactoryBlueprint | None = None) -> int:
        """For given blueprint, return number of geodes made in time."""
        if bp:
            self.set_blueprint(bp)
        LOG.info(
            "Started working on blueprint %d with building costs of %s",
            self.blueprint.bp_id,
            self.blueprint.robot_costs,
        )
        start_resources, start_robots = self.starting_actors_set()
        # time, curr_bots, curr_resource, skipped actions
        que: deque[TimedQueueSearchItem] = deque(
            [(0, start_robots, start_resources, set())]
        )
        best_for_time: defaultdict[int, int] = defaultdict(int)
        LOG.info(
            "Maximum robot counts for each type in blueprint %d are %s",
            self.blueprint.bp_id,
            self.max_robots,
        )
        while que:
            LOG.info("Current scores dict is %s", best_for_time)
            time, curr_bots, curr_res, skipped = que.popleft()
            LOG.debug(
                "At time %d on blueprint %d, got robots %s, resources %s after\
                    skipping %s",
                time,
                self.blueprint.bp_id,
                curr_bots,
                curr_res,
                skipped,
            )
            # make sure
            if (
                time > self.max_minutes
                or best_for_time[time] > curr_res[Resource.GEODE]
            ):
                continue
            LOG.debug(
                "Updating score for time %d from %d to %d, if greater",
                time,
                best_for_time[time],
                curr_res[Resource.GEODE],
            )
            best_for_time[time] = max(best_for_time[time], curr_res[Resource.GEODE])
            options = self.blueprint.makeable_bots(curr_resources=curr_res)
            LOG.debug("Available build options at time %d are %s", time, options)

            que.extend(
                [
                    (time + 1, *item)
                    for item in self.search_build_options(
                        options, curr_res, curr_bots, skipped
                    )
                ]
            )

        # not necessary but do clean up after run
        self.max_robots.clear()
        return best_for_time[self.max_minutes]


class Day19(Day):
    """Day 19 of Advent of Code 2022."""

    def parse(self, puzzle_input: str) -> list[FactoryBlueprint]:
        """Return set of 3d points of lava."""
        return list(map(self._parse_blueprint, puzzle_input.splitlines()))

    @staticmethod
    def _parse_blueprint(line: str) -> FactoryBlueprint:
        values = list(map(int, re.findall(r"\d+", line)))
        return FactoryBlueprint(
            values[0],
            {
                Robot.ORE: {Resource.ORE: values[1]},
                Robot.CLAY: {Resource.ORE: values[2]},
                Robot.OBSIDIAN: {Resource.ORE: values[3], Resource.CLAY: values[4]},
                Robot.GEODE: {Resource.ORE: values[5], Resource.OBSIDIAN: values[6]},
            },
        )

    def part1(self, data: list[FactoryBlueprint]) -> int:
        """Return sum of quality scores of geodes for all blueprints."""
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        satch = Satchel(24)
        total = 0
        for bp in data:
            satch.set_blueprint(bp)
            score = bp.bp_id * satch.blueprint_score()
            total += score
            LOG.info(
                "Blueprint %d gives score %d for total of %d", bp.bp_id, score, total
            )
        return total
        # return sum(bp.bp_id * satch.blueprint_score(bp) for bp in data)

    def part2(self, data: list[FactoryBlueprint]) -> int:
        """With only first 3 blueprints, return product of geodes mined."""
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        satch = Satchel(32)
        LOG.info("The first 3 blueprints are %s", data[:3])
        return prod(satch.blueprint_score(bp) for bp in data[:3])


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 19, 2022
    day = Day19()
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
        assert answers == (33, 3472)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
