#include "day02.hpp"
#include "utils/line_reader.hpp"
#include <iostream>
#include <string>
#include <vector>

void load_numbers(std::vector<int> &numbers, std::string line) {
  std::string buffer;
  for (char c : line) {
    if (c == ' ') {
      if (!buffer.empty()) {
        int number = stoi(buffer);
        numbers.push_back(number);
        buffer.clear();
      }
    } else {
      buffer += c;
    }
  }
  // Don't forget to process the last number if buffer isn't empty
  if (!buffer.empty()) {
    int number = stoi(buffer);
    numbers.push_back(number);
  }
}

bool valid_max_diff(int diff) { return std::abs(diff) <= 3; }

/**
 * Checks if the given vector of numbers is a valid report.
 * A valid report is a vector of numbers that is strictly increasing or
 * decreasing. The difference between two consecutive numbers must be at most 3.
 */
bool is_valid_report(const std::vector<int> &numbers) {
  // assumption that all reports include at least one number
  bool increasing = true;
  bool decreasing = true;

  for (size_t i = 0; i < numbers.size() - 1; i++) {
    if (numbers[i] >= numbers[i + 1]) {
      increasing = false;
    }
    if (numbers[i] <= numbers[i + 1]) {
      decreasing = false;
    }
    int diff = numbers[i + 1] - numbers[i];
    if (!(increasing || decreasing) || !valid_max_diff(diff)) {
      return false;
    }
  }

  return true;
}

bool is_valid_sequence(const std::vector<int> &nums, bool increasing) {
  for (size_t i = 1; i < nums.size(); ++i) {
    if ((increasing && nums[i - 1] >= nums[i]) ||
        (!increasing && nums[i - 1] <= nums[i]) ||
        !valid_max_diff(nums[i - 1] - nums[i])) {
      return false;
    }
  }
  return true;
}
bool is_valid_report_with_remove(const std::vector<int> &numbers) {
  if (numbers.size() <= 2)
    return true;

  for (size_t i = 0; i < numbers.size(); ++i) {
    std::vector<int> temp;
    for (size_t j = 0; j < numbers.size(); ++j) {
      if (j != i) {
        temp.push_back(numbers[j]);
      }
    }
    if (is_valid_sequence(temp, true) || is_valid_sequence(temp, false)) {
      return true;
    }
  }

  return false;
}

namespace aoc {
void solve_day02_part1(const std::string &input_path) {
  int valid_reports = 0;
  std::vector<int> numbers;
  auto end = LineIterator::end();
  for (LineIterator it(input_path); it != end; ++it) {
    const auto &line = *it;
    load_numbers(numbers, line);
    if (is_valid_report(numbers)) {
      valid_reports++;
    }
    numbers.clear();
  }
  std::cout << "\nPart 1: " << valid_reports << std::endl;
}
void solve_day02_part2(const std::string &input_path) {
  int valid_reports = 0;
  std::vector<int> numbers;
  auto end = LineIterator::end();
  for (LineIterator it(input_path); it != end; ++it) {
    const auto &line = *it;
    load_numbers(numbers, line);
    if (is_valid_report_with_remove(numbers)) {
      std::cout << "VALID: " << line << "\n";
      valid_reports++;
    } else {
      std::cout << "INVALID: " << line << "\n";
    }
    numbers.clear();
  }
  std::cout << "\nPart 2: " << valid_reports << std::endl;
}
} // namespace aoc
