"""Advent of Code Day22 problem.

Usage:
    day22.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

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
from collections import deque
import contextlib
from enum import IntEnum
from itertools import product
import logging
from math import sqrt
import os
from pathlib import Path
import re
import sys
from textwrap import dedent
from typing import TYPE_CHECKING
from typing import Any
from typing import Final
from typing import Literal
from typing import NamedTuple
from typing import TypeAlias

if TYPE_CHECKING:
    from collections.abc import Callable
    from collections.abc import Sequence

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

LOG_NAME = "day22"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
            ...#
            .#..
            #...
            ....
    ...#.......#
    ........#...
    ..#....#....
    ..........#.
            ...#....
            .....#..
            .#......
            ......#.

    10R5L5R10L4R5L5"""  # original 10R5L5R10L4R5L5
)


class Facing(IntEnum):
    """Facing direction in Grid view.

    Use int value to make easier to rotate and get scoring.
    """

    RIGHT = 0
    DOWN = 1
    LEFT = 2
    UP = 3

    def __radd__(self, point: Any) -> GridLoc:
        """Return new GridLoc when adding to a GridLoc."""
        if not isinstance(point, GridLoc):
            return NotImplemented
        match self:
            case Facing.RIGHT:
                return GridLoc(point.x_posn + 1, point.y_posn)
            case Facing.LEFT:
                return GridLoc(point.x_posn - 1, point.y_posn)
            case Facing.DOWN:
                return GridLoc(point.x_posn, point.y_posn + 1)
            case Facing.UP:
                return GridLoc(point.x_posn, point.y_posn - 1)
        raise KeyError()

    def __invert__(self) -> Facing:
        """Do two 90 degree turns.

        Returns:
            Facing: Another Facing enum direction
        """
        return Facing((self.value + 2) % 4)


class GridLoc(NamedTuple):
    """Stores a location in Grid as (x,y)."""

    x_posn: int = 0
    y_posn: int = 0

    def __add__(self, other: Any) -> GridLoc:
        """Add two points together to make a new one."""
        if not isinstance(other, GridLoc):
            return NotImplemented
        return GridLoc(self.x_posn + other.x_posn, self.y_posn + other.y_posn)

    def __sub__(self, other: Any) -> GridLoc:
        """Easier to read add of a negation."""
        if not isinstance(other, GridLoc):
            return NotImplemented
        return self + -other

    def __neg__(self) -> GridLoc:
        """Negate both x, y values."""
        return GridLoc(-self.x_posn, -self.y_posn)

    def __mul__(self, scalar: int) -> GridLoc:
        """Multiply by a scalar."""
        return GridLoc(self.x_posn * scalar, self.y_posn * scalar)


class GroveGrid:
    """Grid traversable to get the password for the puzzle.

    Password is made up of row, column and direction facing.
    Row and column are 1 indexed.
    Facing direction is enumerated in the following class.
    """

    _edges: tuple[list[int], list[int], list[int], list[int]]

    def __init__(self, grid: list[str]) -> None:
        """Store basics of object needed to solve Grid layout flat."""
        self.shape = len(grid), max(len(line) for line in grid)
        self.grid = grid
        self.facing = Facing.RIGHT
        self.pos = self._init_edges()

    def _init_edges(self) -> GridLoc:
        """For every edge that would wrap, map to the opposite side."""
        leny, lenx = self.shape
        self._edges = edges = ([0] * leny, [leny] * lenx, [0] * leny, [0] * lenx)

        for y_ind, row in enumerate(self.grid):
            it = iter(enumerate(row))
            minx = next(x for x, char in it if char != " ")
            maxx = next((x for x, char in it if char == " "), len(row)) - 1
            edges[Facing.LEFT][y_ind] = maxx
            edges[Facing.RIGHT][y_ind] = minx
            for x_ind in range(minx, maxx + 1):
                edges[Facing.UP][x_ind] = max(edges[Facing.UP][x_ind], y_ind)
                edges[Facing.DOWN][x_ind] = min(edges[Facing.DOWN][x_ind], y_ind)
        return GridLoc(self._edges[Facing.RIGHT][0], 0)

    def new_pos(self) -> tuple[int, int]:
        """Worthless."""
        return tuple(GridLoc(*self.pos) + self.facing)

    def do_move(self) -> tuple[GridLoc, Facing] | None:
        """Return next point if possible to move to that point."""
        # return False if move can not be done, cut out redundant blocked moves
        col, row = new = self.new_pos()
        edge, char = self._edges[self.facing], " "
        with contextlib.suppress(IndexError):
            if row >= 0 <= col:
                char = self.grid[row][col]
        # adjust for edges
        if char == " ":
            match self.facing:
                case Facing.UP | Facing.DOWN:
                    new = col, edge[col]
                case Facing.LEFT | Facing.RIGHT:
                    new = edge[row], row
            char = self.grid[new[1]][new[0]]
        return None if char == "#" else (GridLoc(*new), self.facing)

    def rotate(self, direction: str) -> Facing:
        """Change facing direction."""
        self.facing = Facing(
            (self.facing.value + (1 if direction == "R" else -1)) % len(Facing)
        )
        return self.facing

    def move(self, repeat: int) -> None:
        """Repeat moves in the current direction until hit a wall."""
        i = 0
        while i < repeat and (match := self.do_move()):
            i += 1
            next_pos, _ = match
            self.pos = next_pos
            # self.dump()

    def get_scoring_factors(self) -> tuple[int, int, int]:
        """Return row, col, facing direction value."""
        return 1000 * (self.pos[1] + 1), 4 * (self.pos[0] + 1), self.facing.value

    def _dump(self, printf: Callable = print) -> None:
        printf("=" * self.shape[1])
        for row, line in enumerate(self.grid):
            for col, char in enumerate(line):
                if self.pos == (row, col):
                    printf("&", end="")
                else:
                    printf(char, end="")
            printf()


CubeEdge: TypeAlias = Literal[
    "a", "b", "c", "d", "e", "f", "g", "h", "j", "k", "m", "n"
]
FaceIndex: TypeAlias = int
EdgeMap: TypeAlias = dict[CubeEdge, FaceIndex]
Direction: TypeAlias = Literal["L", "R"]


class LinTransform(NamedTuple):
    """Helper object to help with Cube wrapping."""

    xs: tuple[int, int, int]
    ys: tuple[int, int, int]
    consts: tuple[int, int, int]

    def __repr__(self) -> str:
        """Debug logic view."""
        return "\n".join(str(field) for field in self)

    def __matmul__(self, rhs: LinTransform) -> LinTransform:
        """Matrix multiplication between two matrices."""
        if not isinstance(rhs, LinTransform):
            return NotImplemented
        cols = list(zip(*rhs, strict=True))
        return type(self)(
            tuple(
                sum(x * c for x, c in zip(self.xs, col, strict=True)) for col in cols
            ),
            tuple(
                sum(y * c for y, c in zip(self.ys, col, strict=True)) for col in cols
            ),
            tuple(
                sum(z * c for z, c in zip(self.consts, col, strict=True))
                for col in cols
            ),
        )

    def __rmatmul__(self, other: GridLoc) -> GridLoc:
        """Return new location when used against a GridLoc."""
        if not isinstance(other, GridLoc):
            return NotImplemented

        vector = (*other, 1)
        # all because I do location by row, col not x, y
        LOG.debug("Doing right matmul with %s and %r", other, self)
        return GridLoc(
            sum(x * c for x, c in zip(self.xs, vector, strict=True)),
            sum(y * c for y, c in zip(self.ys, vector, strict=True)),
        )

    @classmethod
    def rotate(cls, direc: Direction) -> LinTransform:
        """Transform matrices for when a rotation is involved.

        Can stack two of the same to create a 180 flip transform.
        """
        match direc:
            case "L":
                return cls((0, 1, 0), (-1, 0, 0), (0, 0, 1))
            case "R":
                return cls((0, -1, 0), (1, 0, 0), (0, 0, 1))
        raise ValueError("Invalid")

    @classmethod
    def make_transform(cls, x_val: int, y_val: int) -> LinTransform:
        """Turn a GridLoc x, y into a transformation."""
        # looks funky because given Loc points are in row, col which is y,x
        return cls((1, 0, x_val), (0, 1, y_val), (0, 0, 1))


RotR = LinTransform.rotate("R")
RotL = LinTransform.rotate("L")
Rot180 = RotR @ RotR


class GroveCube(GroveGrid):
    """Monkey translation missed that its a 3D cube not a grid.

    Figure out how to wrap the grid around so that the 6 faces are formed.
    So cube is a folded up version of the 2D grid. The start location is on Face 0,
    for a total of 6 faces indexed 0-5. A face index value plus the face
    index value opposite of it equals 5.

    There are a total of 12 edges on a cube, by labeling the edges of the cube
    a specific title we can
    always recreate no matter what input.
    So we start on Face 0, apply an edge to the right side (in grid viewing)
    and then enum out in a clockwise fashion
    we get the edges a, b, c, and d. Then we assign a specific face to be
    attached to that edge of Face 0.
    See CUBE_FACES for the pairings.

    The only other thing is that directions change whenever moving from a
    face to a disconnected face.
    A disconnected face is one that is attached when folded into a cube
    but not in the given grid format.
    When doing this move, the direction inverts (two 90 degree turns).
    """

    CUBE_FACES: Final[Sequence[EdgeMap]] = (
        {"a": 1, "b": 2, "c": 4, "d": 3},  # face 0
        {"e": 3, "f": 5, "g": 2, "a": 0},  # face 1
        {"g": 1, "h": 5, "j": 4, "b": 0},  # so on
        {"m": 4, "n": 5, "e": 1, "d": 0},
        {"j": 2, "k": 5, "m": 3, "c": 0},
        {"k": 4, "h": 2, "f": 1, "n": 3},
    )

    _cube_len: int
    _transform_d: dict[tuple[Facing, GridLoc], tuple[Facing, LinTransform]]

    def _init_edges(self) -> tuple[int, int]:  # noqa: PLR0915, C901
        """Override for new logic Cube form."""
        points = sum(len(line.strip()) for line in self.grid)
        self._cube_len = edge_len = int(sqrt(points // 6))
        faces_h, faces_w = self.shape[0] // edge_len, self.shape[1] // edge_len

        # the given grid is now a faces_h x faces_w grid, only 6 of these are filled in.
        # find the first one from the top left
        start_loc = next(
            GridLoc(c, r)
            for r, c in product(range(faces_h), range(faces_w))
            if self.grid[r * edge_len][c * edge_len] != " "
        )
        # now need to "unfold" the cube and figure out where each edge of lies on a grid
        # edge 'a' connects grid 0 and 1. Grid 0 edge 'a' will be on the right side,
        # so to leave face 0 through
        # edge 'a' (in grid terms)
        # must face right and move forward. We then want to confirm the
        # orientation of that edge and the others of Face 1.
        edge_positions_face: dict[FaceIndex, tuple[GridLoc, dict[CubeEdge, Facing]]] = {
            0: (start_loc, dict(zip(self.CUBE_FACES[0], Facing, strict=True)))
        }
        # also record which edges of each face are not actually connected in grid form
        discon_edges: dict[FaceIndex, set[CubeEdge]] = {
            face: set() for face in range(6)
        }
        seen_faces: set[FaceIndex] = {0}
        queue = deque([0])
        while queue:
            curr_face = queue.popleft()
            curr_loc, edge_dirs = edge_positions_face[curr_face]
            for edge, direction in edge_dirs.items():
                connection_loc = curr_loc + direction
                test_char = " "
                with contextlib.suppress(IndexError):
                    # technically not connected through bounds of grid
                    if connection_loc.x_posn >= 0 <= connection_loc.y_posn:
                        test_char = self.grid[connection_loc.y_posn * edge_len][
                            connection_loc.x_posn * edge_len
                        ]
                if test_char == " ":
                    discon_edges[curr_face].add(edge)
                    continue

                conn_face = self.CUBE_FACES[curr_face][edge]
                if conn_face in seen_faces:
                    continue
                seen_faces.add(conn_face)
                new_edges = list(self.CUBE_FACES[conn_face])
                # rotate to match grid alignment
                # if to get to this connected face we went right,
                # then to get off through this edge its LEFT
                delta = (new_edges.index(edge) - ~direction) % 4
                rotated = new_edges[delta:] + new_edges[:delta]
                LOG.debug(
                    "Found new connected face %d for face %d through edge %r in\
                        direction %s location %s",
                    conn_face,
                    curr_face,
                    edge,
                    direction,
                    connection_loc,
                )
                edge_positions_face[conn_face] = (
                    connection_loc,
                    dict(zip(rotated, Facing, strict=True)),
                )
                LOG.debug(
                    "Adding task to check edges with in order of %s",
                    dict(zip(rotated, Facing, strict=True)),
                )
                queue.append(conn_face)

        LOG.debug("Faces begin dict %s", edge_positions_face)
        # for each disconnected edge, have to move position to
        # appropriate location on new face
        self._transform_d = td = {}
        for face, discon in discon_edges.items():
            LOG.debug("Face %d has disconnected edges of %s", face, discon)
            for edge in discon:
                discon_face = self.CUBE_FACES[face][edge]
                main_loc, main_edge_dir = edge_positions_face[face]
                discon_loc, discon_edge_dir = edge_positions_face[discon_face]
                LOG.debug(
                    "Working on edge %r on face %d, which begins at position %s",
                    edge,
                    face,
                    main_loc,
                )
                # direction off face through edge
                main_leave_dir, discon_leave_dir = (
                    main_edge_dir[edge],
                    discon_edge_dir[edge],
                )
                # going to generate points for each edge in clockwise order,
                # so make sure each is in proper corner
                # RIGHT -> top right, DOWN -> bottom right etc
                main_corner = (main_loc * edge_len) + GridLoc(
                    edge_len - 1
                    if main_leave_dir in {Facing.DOWN, Facing.RIGHT}
                    else 0,
                    edge_len - 1 if main_leave_dir in {Facing.DOWN, Facing.LEFT} else 0,
                )
                # one corner forward, since they fold together its flipped
                discon_corner = (discon_loc * edge_len) + GridLoc(
                    edge_len - 1
                    if discon_leave_dir in {Facing.UP, Facing.RIGHT}
                    else 0,
                    edge_len - 1
                    if discon_leave_dir in {Facing.DOWN, Facing.RIGHT}
                    else 0,
                )
                trans = LinTransform.make_transform(*-main_corner)
                match (main_leave_dir - ~discon_leave_dir) % 4:
                    case 1:
                        trans = RotL @ trans
                    case 2:
                        trans = Rot180 @ trans
                    case 3:
                        trans = RotR @ trans
                trans = LinTransform.make_transform(*discon_corner) @ trans
                match main_leave_dir:
                    case Facing.RIGHT:
                        points = (main_corner + GridLoc(0, y) for y in range(edge_len))
                    case Facing.DOWN:
                        points = (main_corner + GridLoc(-x, 0) for x in range(edge_len))
                    case Facing.LEFT:
                        points = (main_corner + GridLoc(0, -y) for y in range(edge_len))
                    case Facing.UP:
                        points = (main_corner + GridLoc(x, 0) for x in range(edge_len))
                for point in points:
                    td[main_leave_dir, point] = (~discon_leave_dir, trans)
                    LOG.debug(
                        "Added (%s, %s) to the transform dict", main_leave_dir, point
                    )
        return edge_positions_face[0][0] * edge_len

    def do_move(self) -> tuple[GridLoc, Facing] | None:
        """Override for Cube form.

        If point is on a disconnected edge it needs to go through a
        matrix multiplication to get the new location properly.
        """
        if match := self._transform_d.get((self.facing, self.pos)):
            LOG.info(
                "Position %s is on an edge, going to be using %s to switch over", *match
            )
            facing, trans = match
            x, y = new = self.pos @ trans
            LOG.debug("Using transformation at %s yielded (%d,%d)", self.pos, x, y)
        else:
            LOG.info(
                "Position %s is in a face, moving one spot in direction %s",
                self.pos,
                self.facing,
            )
            x, y = new = self.pos + self.facing
            facing = self.facing
        # x and y are guaranteed to be a valid cube face in the map
        return None if self.grid[y][x] == "#" else (new, facing)

    def move(self, repeat: int) -> None:
        """Override move for Cube.

        Now the facing can get changed by doing a move.
        """
        i = 0
        while i < repeat and (match := self.do_move()):
            i += 1
            new_pos, new_facing = match
            LOG.info(
                "Successfully completed step (%d / %d) and ended at %s facing %s",
                i,
                repeat,
                new_pos,
                new_facing,
            )
            self.pos = new_pos
            self.facing = new_facing
            # self.dump()


class Day22(Day):
    """Day 22 of Advent of Code 2022."""

    def parse(self, puzzle_input: str) -> tuple[list[str], str]:
        """Return seperated lines."""
        lines = puzzle_input.splitlines()
        sep_index = lines.index("")
        return lines[:sep_index], lines[sep_index + 1]

    def _execute_steps(self, grove: GroveGrid | GroveCube, steps: str) -> int:
        for match in re.finditer(r"(\d+|[LR])", steps):
            LOG.info("From instructions working on step %r", match.group(0))
            if match.group(0).isnumeric():
                # have to move so many times
                grove.move(int(match.group(0)))
            else:
                rotate_ret = grove.rotate(match.group(0))
                LOG.debug(
                    "Current direction is now %r at position %s", rotate_ret, grove.pos
                )

        scores = grove.get_scoring_factors()
        LOG.info("Got scoring criteria of col %d, row %d, facing value %d", *scores)
        return sum(scores)

    def part1(self, data: tuple[list[str], str]) -> int:
        """Return value for the password that Grid wants."""
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        gg = GroveGrid(data[0])
        return self._execute_steps(gg, data[1])

    def part2(self, data: tuple[list[str], str]) -> int:
        """Return value for the password that Cube wants."""
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        # grid is a hardcoded 15,000 tiles 6 - 50x50 faces
        gc = GroveCube(data[0])
        LOG.info("Starting with position of %s", gc.pos)
        LOG.debug("Have a deep dict of %s", gc._transform_d)
        return self._execute_steps(gc, data[1])


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 22, 2022
    day = Day22()
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
        assert answers == (6032, 5031)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
