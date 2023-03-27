# Standard Library

# External Party
from aocd import get_data
from aocd import submit

# My Modules
from common.template import Day


class Day3(Day):
    """Day 3 of Advent of Code 2021."""

    def parse(self, data_input: str) -> list[str]:
        """Given input return each line."""
        return data_input.split("\n")

    @staticmethod
    def bit_criteria_oxy(bits: list[str], pos: int) -> int:
        """Return most common bit for a given position from bits array.

        If both bits are equally as common it will return 1.

        Parameters
        ----------
        bits : List[str]
            array of binary representations
        pos : int
            position of bit to look at in binary number

        Returns:
        -------
        int
            the most common bit, 1 or 0

        """
        ones = sum(1 for line in bits if line[pos] == "1")
        return 1 if ones * 2 >= len(bits) else 0

    @staticmethod
    def bit_criteria_carb(bits: list[str], pos: int) -> int:
        """Return most common bit for a given position from bits array.

        If both bits are equally as common it will return 0.

        Parameters
        ----------
        bits : List[str]
            array of binary representations
        pos : int
            position of bit to look at in binary number

        Returns:
        -------
        int
            the most common bit, 1 or 0

        """
        ones = sum(1 for line in bits if line[pos] == "1")
        return 1 if ones * 2 < len(bits) else 0

    def part1(self, data: list[str]) -> int:
        """Return the product of the gamma and epsilon values.

        Gamma is made up of the most common bits of each position for all
        binary numbers in the input data. Epsilon is 1's complement of Gamma.
        E.g::
            1010, 0000, 1111 -> Gamma: int(1010), Epsilon: int(0101) -> 50

        Parameters
        ----------
        data : List[str]
            array of binary numbers represented in string format

        Returns:
        -------
        int

        """
        line_len = max(len(line) for line in data)
        assert all(line_len == len(line) for line in data)
        gamma = ""
        epsilon = ""
        for i in range(len(data[0])):
            if self.bit_criteria_oxy(data, i):
                gamma += "1"
                epsilon += "0"
            else:
                gamma += "0"
                epsilon += "1"
        return int(gamma, base=2) * int(epsilon, base=2)

    def part2(self, data: list[str]) -> int:
        """Return the life system rating.

        Life system rating = oxygen rating * carbon rating. A rating is found
        by finding a certain bit in a position of the remaining data and
        filtering the input list down to only binary numbers that satisfy
        that condition. Oxygen is based on most common bit with 1s having
        precedent in a 50/50.
        Carbon is based on lost common bit with 0s having precedent in a 50/50.
        The filtering ends when there is only 1 number left for each one.

        Parameters
        ----------
        data : List[str]
            input data of binary numbers represented in string type

        Returns:
        -------
        int

        """
        oxy_list, carb_list = data.copy(), data.copy()
        oxy_crit, carb_crit = [], []
        for pos in range(len(data[0])):
            if len(oxy_list) > 1:
                oxy_crit.append(self.bit_criteria_oxy(oxy_list, pos))
                # late binding issue doesn't seem to be present here
                # we want this global changing pos
                oxy_list = list(
                    filter(lambda x: x[pos] == str(oxy_crit[pos]), oxy_list)
                )
            if len(carb_list) > 1:
                carb_crit.append(self.bit_criteria_carb(carb_list, pos))
                carb_list = list(
                    filter(lambda x: x[pos] == str(carb_crit[pos]), carb_list)
                )

        assert len(oxy_list) == 1
        assert len(carb_list) == 1
        return int(oxy_list[0], base=2) * int(carb_list[0], base=2)


if __name__ == "__main__":
    day = Day3()
    answers = day.solve(get_data(day=3, year=2021))
    print(answers[1])
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=3, year=2021, part=part)
