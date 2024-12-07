#include "day07.hpp"
#include "utils/line_reader.hpp"
#include <iostream>
#include <regex>
#include <string>
#include <vector>

/**
 A given line of input looks like
  10: 1 1 5

Goal being to use + and * to evaluate the expression on the right of the colon
to create the number on the left.
 */

// regex to parse the numbers in the line
// 10: 1 1 5
std::regex number_regex("\\d+");

/**
 * Return a line as a vector of integers. First number is the number on the left
 * of colon.
 */
std::vector<long> parse_line(const std::string &line) {
  std::vector<long> numbers;
  for (std::sregex_iterator it(line.begin(), line.end(), number_regex);
       it != std::sregex_iterator(); ++it) {
    numbers.push_back(std::stol(it->str()));
  }
  return numbers;
}

bool backtrack_operators_1(std::vector<long> &numbers, long target, size_t idx,
                           long running_total) {
  if (idx == numbers.size()) {
    return running_total == target;
  }
  return backtrack_operators_1(numbers, target, idx + 1,
                               running_total + numbers[idx]) ||
         backtrack_operators_1(numbers, target, idx + 1,
                               running_total * numbers[idx]);
}

long concatenate_numbers(long a, long b) {
  // Find number of digits in b
  long temp = b;
  long multiplier = 1;
  while (temp > 0) {
    multiplier *= 10;
    temp /= 10;
  }
  return a * multiplier + b;
}

bool backtrack_operators_2(std::vector<long> &numbers, long target, size_t idx,
                           long running_total) {
  if (idx == numbers.size()) {
    return running_total == target;
  }
  return backtrack_operators_2(numbers, target, idx + 1,
                               running_total + numbers[idx]) ||
         backtrack_operators_2(numbers, target, idx + 1,
                               running_total * numbers[idx]) ||
         backtrack_operators_2(
             numbers, target, idx + 1,
             concatenate_numbers(running_total, numbers[idx]));
}

namespace aoc {
void solve_day07_part1(const std::string &input_path) {
  auto end = LineIterator::end();
  long valid_equation_total = 0;
  for (LineIterator it(input_path); it != end; ++it) {
    const auto &line = *it;
    std::vector<long> numbers = parse_line(line);
    if (numbers.empty()) {
      continue;
    }
    long target = numbers[0];
    // make operands vector by consuming iterator
    std::vector<long> operands(numbers.begin() + 1, numbers.end());
    // can't start at index 0, as we have no good iniital value
    // start at 1 for running, then our addition operation is off
    // start at 0 for running, then multiplication is always 0
    if (backtrack_operators_1(operands, target, 1, operands[0])) {
      valid_equation_total += target;
    }
  }
  std::cout << "\nPart 1: " << valid_equation_total << std::endl;
}
void solve_day07_part2(const std::string &input_path) {
  auto end = LineIterator::end();
  long valid_equation_total = 0;
  for (LineIterator it(input_path); it != end; ++it) {
    const auto &line = *it;
    std::vector<long> numbers = parse_line(line);
    if (numbers.empty()) {
      continue;
    }
    long target = numbers[0];
    // make operands vector by consuming iterator
    std::vector<long> operands(numbers.begin() + 1, numbers.end());
    // can't start at index 0, as we have no good iniital value
    // start at 1 for running, then our addition operation is off
    // start at 0 for running, then multiplication is always 0
    if (backtrack_operators_2(operands, target, 1, operands[0])) {
      valid_equation_total += target;
    }
  }
  std::cout << "\nPart 2: " << valid_equation_total << std::endl;
}
} // namespace aoc
