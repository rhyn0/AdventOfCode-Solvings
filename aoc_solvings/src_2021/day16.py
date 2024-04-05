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
from collections.abc import Callable
from dataclasses import dataclass
from functools import reduce
import logging
import operator
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

LOG_NAME = "day16"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    9C0141080250320F1802104A08"""
)

VALBIT_ID = 4
PACKET_VERSION_LEN = 3
PACKET_TYPE_LEN = 3
HEADER_LEN = 6


@dataclass
class BITSPacket:
    packet_version: int
    packet_type: int
    packet_length: int
    subpackets: list[BITSPacket]
    op: Callable[..., int] | None = None
    value: int = 0

    def sum_versions(self) -> int:
        """Return sum of all packet versions in this packet."""
        return self.packet_version + sum(
            [pkt.sum_versions() for pkt in self.subpackets]
        )

    def perform_op(self) -> int:
        if self.op is None:
            return self.value
        operands = [pkt.perform_op() for pkt in self.subpackets]
        LOG.info("Performing operation %s on operands %r", self.op, operands)
        return int(self.op(*operands))

    @staticmethod
    def sum_packet(
        packet_version: int,
        packet_type: int,
        packet_len: int,
        subpackets: list[BITSPacket],
    ) -> BITSPacket:
        return BITSPacket(
            packet_version,
            packet_type,
            packet_len,
            subpackets,
            op=lambda *args: reduce(lambda x, y: x + y, args),
        )

    @staticmethod
    def prod_packet(
        packet_version: int,
        packet_type: int,
        packet_len: int,
        subpackets: list[BITSPacket],
    ) -> BITSPacket:
        return BITSPacket(
            packet_version,
            packet_type,
            packet_len,
            subpackets,
            op=lambda *args: reduce(lambda x, y: x * y, args),
        )

    @staticmethod
    def minimum_packet(
        packet_version: int,
        packet_type: int,
        packet_len: int,
        subpackets: list[BITSPacket],
    ) -> BITSPacket:
        return BITSPacket(
            packet_version,
            packet_type,
            packet_len,
            subpackets,
            op=lambda *args: min(args),
        )

    @staticmethod
    def maximum_packet(
        packet_version: int,
        packet_type: int,
        packet_len: int,
        subpackets: list[BITSPacket],
    ) -> BITSPacket:
        return BITSPacket(
            packet_version,
            packet_type,
            packet_len,
            subpackets,
            op=lambda *args: max(args),
        )

    @staticmethod
    def greater_than_packet(
        packet_version: int,
        packet_type: int,
        packet_len: int,
        subpackets: list[BITSPacket],
    ) -> BITSPacket:
        return BITSPacket(
            packet_version,
            packet_type,
            packet_len,
            subpackets,
            op=operator.gt,
        )

    @staticmethod
    def less_than_packet(
        packet_version: int,
        packet_type: int,
        packet_len: int,
        subpackets: list[BITSPacket],
    ) -> BITSPacket:
        return BITSPacket(
            packet_version,
            packet_type,
            packet_len,
            subpackets,
            op=operator.lt,
        )

    @staticmethod
    def equal_packet(
        packet_version: int,
        packet_type: int,
        packet_len: int,
        subpackets: list[BITSPacket],
    ) -> BITSPacket:
        return BITSPacket(
            packet_version,
            packet_type,
            packet_len,
            subpackets,
            op=operator.eq,
        )


def parse_packet_headers(packet: str) -> tuple[int, int]:
    """Parse header of packet and return version and type.

    First 3 bits are version and next 3 bits are type.
    """
    pkt_vers = int(packet[:PACKET_VERSION_LEN], 2)
    pkt_type = int(packet[PACKET_VERSION_LEN : PACKET_VERSION_LEN + PACKET_TYPE_LEN], 2)
    return pkt_vers, pkt_type


def parse_packet(packet: str) -> BITSPacket:
    """Return BITSPacket from packet string."""

    def parse_value_packet(version: int, packet_type: int, packet: str) -> BITSPacket:
        # type is always 4 here, but pass in anyway
        idx = 0
        packet_len = len(packet)
        value = ""
        while idx < packet_len and packet[idx] == "1":
            idx += 1
            # bits for values come in groups of 5, with first bit
            # being identifier on if more bits
            value += packet[idx : idx + 4]
            idx += 4
        value += packet[idx + 1 : idx + 5]
        idx += 5  # for the end case
        # TODO: do we need the value ever?
        # add 6 for the header
        LOG.info("Value packet was of length %d", idx + 6)
        return BITSPacket(version, packet_type, idx + 6, [], value=int(value, 2))

    LOG.debug("Parsing packet from input %r", packet)
    idx = 0
    pkt_vers, pkt_type = parse_packet_headers(packet)
    LOG.debug("Packet version: %s, Packet type: %s", pkt_vers, pkt_type)
    idx += HEADER_LEN
    if pkt_type == VALBIT_ID:
        LOG.info("Parsing value packet from input %r", packet)
        return parse_value_packet(pkt_vers, pkt_type, packet[idx:])
    subpackets = []
    idx += 1  # for operator type bit
    if packet[idx - 1] == "1":
        # 11 bits for the number of subpackets contained
        num_subpackets = int(packet[idx : idx + 11], 2)
        LOG.debug("Found Operator NumSubpacket: %s", num_subpackets)
        idx += 11
        for i in range(num_subpackets):
            subpacket = parse_packet(packet[idx:])
            subpackets.append(subpacket)
            LOG.debug("Subpacket %d was of length %d", i, subpacket.packet_length)
            idx += subpacket.packet_length
    else:
        # 15 bits for the length of all subpackets contained
        subpacket_len = int(packet[idx : idx + 15], 2)
        idx += 15
        original_idx = idx
        LOG.debug("Found Operator TotalLengthPacket: %s", subpacket_len)
        while idx - original_idx < subpacket_len:
            subpacket = parse_packet(packet[idx:])
            subpackets.append(subpacket)
            LOG.debug("Subpacket at %d was of length %d", idx, subpacket.packet_length)
            idx += subpacket.packet_length

    type_map = {
        0: BITSPacket.sum_packet,
        1: BITSPacket.prod_packet,
        2: BITSPacket.minimum_packet,
        3: BITSPacket.maximum_packet,
        5: BITSPacket.greater_than_packet,
        6: BITSPacket.less_than_packet,
        7: BITSPacket.equal_packet,
    }
    return type_map[pkt_type](pkt_vers, pkt_type, idx, subpackets)


class Day16(Day):
    """Day 16 of Advent of Code 2021."""

    def parse(self, puzzle_input: str) -> str:
        """Return bin string with no extra spaces."""
        return f"{int(puzzle_input.strip(), 16):0>{len(puzzle_input) * 4}b}"

    def part1(self, data: str) -> int:
        """Return sum of the packet versions for all packets in data."""
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        bit_packet = parse_packet(data)
        return bit_packet.sum_versions()

    def part2(self, data: str) -> int:
        """Return count of max element minus count of min element after insertions."""
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        bit_packet = parse_packet(data)
        return bit_packet.perform_op()


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
    answers = day.solve(data, parts=args["--parts"])
    print(answers)
    if args["--example"]:
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
