"""Advent of Code Day15 problem.

Usage:
    day15.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

Options:
    --example       Use example input rather than running personal input.
    --local         Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose       Use python logging to get verbose output of what is going
                    on in a log file.
    --quiet         Disable logging for example mode.
    --parts PART    Do only specified part, options are 'a', 'b', or 'ab'. [default: ab]
"""
from __future__ import annotations

# Standard Library
from dataclasses import dataclass
from itertools import product
import logging
import os
from pathlib import Path
import re
import sys
from textwrap import dedent
from typing import TYPE_CHECKING
from typing import Any

if TYPE_CHECKING:
    from collections.abc import Generator
    from collections.abc import Iterator

# External Party
from aocd import get_data
from aocd import submit
from docopt import docopt

try:
    # My Modules
    from common.template import AnswerNotFoundError
    from common.template import Day
except ImportError:
    sys.path.insert(0, os.path.dirname(sys.path[0]))
    # My Modules
    from common.template import AnswerNotFoundError
    from common.template import Day

LOG_NAME = "day15"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3"""
)


@dataclass(frozen=True)
class Point:
    """Class to store 2D data."""

    x_pos: int
    y_pos: int

    def __add__(self, other: Any) -> Point:
        """Add 2 Point together, return a new one."""
        if not isinstance(other, Point):
            return NotImplemented
        return Point(self.x_pos + other.x_pos, self.y_pos + other.y_pos)

    def __sub__(self, other: Any) -> Point:
        """Subtract other Point from this, return a new one."""
        if not isinstance(other, Point):
            return NotImplemented
        return Point(self.x_pos - other.x_pos, self.y_pos - other.y_pos)

    def __iter__(self) -> Generator[int, None, None]:
        """Yield over x, y."""
        yield self.x_pos
        yield self.y_pos

    def manhattan_dist(self, other: Point) -> int:
        """Calculate Manhattan Distance to other Point."""
        diff = self - other
        return abs(diff.x_pos) + abs(diff.y_pos)


class Grid:
    """Storage object for calculation methods relevant to problem."""

    def __init__(self, sensor_beacon: list[tuple[Point, Point]]) -> None:
        """Initialize grid with tuples of Sensor, Beacon."""
        self.sensor_beacons = sensor_beacon
        (
            self.sensor_ranges,
            self.beacon_set,
            self.left_bound,
            self.right_bound,
        ) = self._sensor_ranges(sensor_beacon)

    def _build_coverage_intervals(self, row: int) -> list[list[int]]:
        sensors_in_range = [
            (sensor_point, max_dist)
            for sensor_point, max_dist in self.sensor_ranges
            if abs(sensor_point.y_pos - row) <= max_dist
        ]

        intervals = []
        for sensor, dist in sensors_in_range:
            x_dist_avail = dist - abs(sensor.y_pos - row)
            intervals.append([sensor.x_pos - x_dist_avail, sensor.x_pos + x_dist_avail])

        return intervals

    def merge_intervals(self, row: int) -> list[list[int]]:
        """For a requested row, return intervals that columns are covered.

        Merges intervals before returning.

        Args:
            row (int): Target row to check

        Returns:
            list[list[int]]: List of intervals, represented as list of 2 ints
        """
        intervals = sorted(self._build_coverage_intervals(row))
        latest = [intervals[0]]

        for interval in intervals[1:]:
            if latest[-1][0] <= interval[0] <= latest[-1][1]:
                latest[-1][1] = max(interval[1], latest[-1][1])
            else:
                latest.append(interval)

        return latest

    @staticmethod
    def generate_rows(sensor, distance: int, upper_bound: int) -> Iterator[int]:
        """Generate valid rows to check for range covered by a sensor."""
        for x_dist, dir_y in product(range(distance + 2), (-1, 1)):
            y_dist_avail = (distance + 1) - x_dist
            new_y = sensor.y_pos + y_dist_avail * dir_y

            if not (0 <= new_y <= upper_bound):
                continue
            yield new_y

    @staticmethod
    def find_gap_point(row_coverage: list[list[int]]) -> int | None:
        """Iterate over current range and find gap, if there is one."""
        for i, interval in enumerate(row_coverage[1:], start=0):
            if interval[0] - row_coverage[i][1] > 1:
                # found gap in coverage, must be this point
                return interval[0] - 1
        return None

    def check_sensors_perimeters(self, xy_upper_bound: int = 4_000_000) -> Point:
        """Find spot that is not covered by sensor sweep in boundary range.

        Sweep the perimeters for each sensor until we find a spot that is not covered.

        Args:
            xy_upper_bound (int, optional): Maximum square to check inside of.
                Defaults to 4_000_000.

        Raises:
            ValueError: If grid has no sensor gap

        Returns:
            Point: Point at which gap occurs
        """
        for sensor, dist in self.sensor_ranges:
            # need to check dist+1 to be perimeter
            for new_y in self.generate_rows(sensor, dist, xy_upper_bound):
                row_coverage = self.merge_intervals(new_y)
                if gap_point := self.find_gap_point(row_coverage):
                    return Point(gap_point, new_y)

        raise AnswerNotFoundError()

    def tuning_frequency(self, pt: Point) -> int:
        """Return special tuning frequency of a point."""
        return pt.x_pos * 4_000_000 + pt.y_pos

    def _sensor_ranges(
        self, sensor_beacons: list[tuple[Point, Point]]
    ) -> tuple[list[tuple[Point, int]], set[Point], int, int]:
        sensor_ranges = []
        beacon_set = set()
        left_bound = right_bound = 0
        for sensor, beacon in sensor_beacons:
            sb_dist = sensor.manhattan_dist(beacon)
            sensor_ranges.append((sensor, sb_dist))
            beacon_set.add(beacon)
            left_bound = min(left_bound, sensor.x_pos - sb_dist, beacon.x_pos - sb_dist)
            right_bound = max(
                right_bound, sensor.x_pos + sb_dist, beacon.x_pos + sb_dist
            )

        return sensor_ranges, beacon_set, left_bound, right_bound


class Day15(Day):
    """Day 15 of Advent of Code 2022."""

    def __init__(self) -> None:
        """Store default values."""
        super().__init__()
        self._checking_row = 2_000_000
        self._max_col = self._max_depth = 4_000_000

    def parse(self, puzzle_input: str) -> list[tuple[Point, Point]]:
        """Return sensor, beacon pairs."""
        pttrn = re.compile(r"x=(-?\d+),\sy=(-?\d+)", re.I)
        return [
            tuple(Point(*map(int, x)) for x in pttrn.findall(line))
            for line in puzzle_input.splitlines()
        ]

    @staticmethod
    def _manhat_dist(x1: int, y1: int, x2: int, y2: int) -> int:
        return abs(x1 - x2) + abs(y1 - y2)

    def part1(self, data: list[tuple[Point, Point]]) -> int:
        """Return number of known non-sensor points in target column."""
        LOG.info("-" * 20 + "starting part1" + "-" * 20)
        grid = Grid(data)
        LOG.info(
            "Generated %d Manhattan distances with left and right bounds of %d, %d.",
            len(grid.sensor_ranges),
            grid.left_bound,
            grid.right_bound,
        )
        LOG.debug("Manhattan distances %s", grid.sensor_ranges)

        LOG.info("Checking covered spots in row %d", self._checking_row)
        intervals = grid.merge_intervals(self._checking_row)
        return sum(interval[1] - interval[0] + 1 for interval in intervals) - len(
            [beacon for beacon in grid.beacon_set if beacon.y_pos == self._checking_row]
        )

    def part2(self, data: list[tuple[Point, Point]]) -> int:
        """Return tuning frequency of sensor gap spot."""
        LOG.info("-" * 20 + "starting part2" + "-" * 20)
        grid = Grid(data)
        pt = grid.check_sensors_perimeters(max(self._max_col, self._max_depth))

        return grid.tuning_frequency(pt)


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 15, 2022
    day = Day15()
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
        day._checking_row = 10
        day._max_col = day._max_depth = 20
        grid = EXAMPLE
    elif args["--local"]:
        grid = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        grid = get_data(day=DAY, year=YEAR)
    answers = day.solve(grid, parts=args["--parts"][0])
    print(answers)
    if args["--example"]:
        assert answers == (26, 56000011)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
