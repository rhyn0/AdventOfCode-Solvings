"""Advent of Code 2021 Day16 problem.

Usage:
    day16.py [--example [--quiet] | --local] [--verbose] [--parts=<char>]

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
from abc import ABC
from abc import abstractmethod
import logging
import os
from pathlib import Path
import sys
from textwrap import dedent
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Callable

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
    A0016C880162017C3686B18A3D4780"""
)

VALBIT_ID = 4


class BITSPacket(ABC):
    version: int
    pkt_type: int

    def __init__(self, ver: int, bit_type: int) -> None:
        self.version = ver
        self.pkt_type = bit_type

    def __repr__(self) -> str:
        return f"BITSPacket(version={self.version}, type={self.pkt_type})"

    @abstractmethod
    def evaluate(self) -> int:
        pass

    @classmethod
    @abstractmethod
    def parse_from(
        cls, version: int, pkt_type: int, value_part: str
    ) -> tuple[BITSPacket, int]:
        pass


class ValBITS(BITSPacket):
    value: int

    def __init__(self, ver: int, bit_type: int, value: int) -> None:
        super().__init__(ver, bit_type)
        self.value = value

    def evaluate(self) -> int:
        return self.value

    @classmethod
    def parse_from(
        cls, version: int, pkt_type: int, value_part: str
    ) -> tuple[BITSPacket, int]:
        """Return the created BITSPacket and how long the whole packet was."""
        val_str = ""
        idx = 0
        while value_part[idx] != "0":
            val_str += value_part[idx + 1 : idx + 5]
            idx += 5
        val_str += value_part[idx + 1 : idx + 5]
        return (
            ValBITS(version, pkt_type, int(val_str, 2)),
            idx + 5 + 6,
        )  # last index + 5 to end of pentad + header


class OpBITS(BITSPacket):
    subpackets: list[BITSPacket]
    op: Callable[..., int]

    def add_subpacket(self, pkt: BITSPacket) -> None:
        self.subpackets.append(pkt)

    def evaluate(self) -> int:
        return self.op(*self.subpackets)

    @classmethod
    def parse_from(
        cls, version: int, pkt_type: int, value_part: str
    ) -> tuple[OpBITS, int]:
        LOG.debug("Operator Pkt type is %r", value_part[0])
        args = (version, pkt_type, value_part[1:])
        return (
            LenOpBITS.parse_from(*args)
            if value_part[0] == "0"
            else PktOpBITS.parse_from(*args)
        )


class LenOpBITS(OpBITS):
    total_pkt_len: int

    def __init__(self, ver: int, bit_type: int, pkt_len: int) -> None:
        super().__init__(ver, bit_type)
        self.total_pkt_len = pkt_len

    @classmethod
    def parse_from(
        cls, version: int, pkt_type: int, value_part: str
    ) -> tuple[OpBITS, int]:
        subpkt_len = int(value_part[:15], 2)
        LenOpBITS(version, pkt_type, subpkt_len)


class PktOpBITS(OpBITS):
    total_pkts: int

    def __init__(self, ver: int, bit_type: int, pkt_len: int) -> None:
        super().__init__(ver, bit_type)
        self.total_pkts = pkt_len

    @classmethod
    def parse_from(
        cls, version: int, pkt_type: int, value_part: str
    ) -> tuple[OpBITS, int]:
        return super().parse_from(version, pkt_type, value_part)


class BITSPacketParser:
    # value packet of one set of bits is the smallest a packet can ever be
    MIN_PKT_LEN = 11

    @classmethod
    def parse_packet_header(cls, string: str) -> tuple[int, int]:
        """Get details of the first packet found in the string.

        Args:
            string (str): input bit string

        Returns:
            tuple[int, int, int]: version, type id, len of this packet
        """
        # TODO: remove pkt_len from return, its causing double parsing in this case
        version = int(string[:3], 2)
        pkt_type = int(string[3:6], 2)
        idx = 6
        if pkt_type == VALBIT_ID:
            while string[idx] == "1":
                idx += 5
            return version, pkt_type

        idx += 1
        if string[6] == "0":
            idx += 15
            return version, pkt_type
        LOG.debug("Given a packet num operator packet")
        # some number of subpackets that we don't know the length of
        return version, pkt_type

    @classmethod
    def parse_packets(cls, bin_string: str) -> tuple[int, list[BITSPacket]]:
        """Parse packets from the binary string.

        Args:
            bin_string (str): binary string

        Returns:
            tuple[int, list[BITSPacket]]: versions sum, parsed packets
        """
        if not bin_string:
            return 0, []
        str_len = len(bin_string)
        index, vers, pkts = 0, 0, []
        LOG.debug("Parsing packets from given string of %r", bin_string)
        while index < str_len:
            LOG.debug(
                "Looking for packet in bin_string starting at index %d.\
                    Bin string is of length %d",
                index,
                str_len,
            )
            if index >= str_len - cls.MIN_PKT_LEN:
                # no valid header to be found now
                break
            pkt_ver, pkt_type = cls.parse_packet_header(bin_string[index:])
            LOG.debug(
                "Current packet is a %r of version %d and type %d",
                ValBITS.__name__ if pkt_type == VALBIT_ID else OpBITS.__name__,
                pkt_ver,
                pkt_type,
            )
            index += 6
            LOG.debug("Index was bumped past the header to be %d", index)
            if pkt_type == VALBIT_ID:
                LOG.debug(
                    "Finding a literal value from the string starting at index %d",
                    index,
                )
                new_pkt, pkt_len = ValBITS.parse_from(
                    pkt_ver, pkt_type, bin_string[index:]
                )
                pkts.append(new_pkt)
                LOG.debug("Created value packet of length %d - %r", pkt_len, pkts[-1])
                vers += pkt_ver
            else:
                new_pkt, pkt_len = OpBITS.parse_from(
                    pkt_ver, pkt_type, bin_string[index:]
                )
                # TODO - OpBITS sum_version?
                sub_vers_sum, new_pkts = cls.parse_packets(bin_string[index:])
                vers += sub_vers_sum
                pkts.append(new_pkt)
                index += pkt_len
            # might be double counting the version of the operator pkt
            index += pkt_len

        return vers, pkts


class Day16(Day):
    """Day 16 of Advent of Code 2021."""

    def parse(self, puzzle_input: str) -> str:
        """Return bin string with no extra spaces."""
        return f"{int(puzzle_input.strip(), 16):0>{len(puzzle_input) * 4}b}"

    def part1(self, data: str) -> int:
        """Return sum of the packet versions for all packets in data."""
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        total_vers, pkts = BITSPacketParser.parse_packets(data)
        LOG.info("Created %d packets which are %s", len(pkts), pkts)
        return total_vers

    def part2(self, data: list[list[int]]) -> int:
        """Return count of max element minus count of min element after insertions."""
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        return -1


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 16, 2021
    day = Day16()
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
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
