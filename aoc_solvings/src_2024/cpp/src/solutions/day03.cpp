#include "day03.hpp"
#include "utils/line_reader.hpp"
#include <iostream>
#include <regex>
#include <string>

int evaluate_mul_expression(const std::smatch &match) {
  int num1 = std::stoi(match[1]);
  int num2 = std::stoi(match[2]);
  return num1 * num2;
}

namespace aoc {
void solve_day03_part1(const std::string &input_path) {
  std::string line;
  auto end = LineIterator::end();
  int running_sum = 0;
  std::regex pattern("mul\\((\\d{1,3}),(\\d{1,3})\\)");

  for (LineIterator it(input_path); it != end; ++it) {
    const auto &line = *it;
    auto words_begin = std::sregex_iterator(line.begin(), line.end(), pattern);
    auto words_end = std::sregex_iterator();
    for (std::sregex_iterator it = words_begin; it != words_end; ++it) {
      std::smatch match = *it;
      running_sum += evaluate_mul_expression(match);
    }
  }
  std::cout << "\nPart 1: " << running_sum << std::endl;
}
void solve_day03_part2(const std::string &input_path) {
  std::string line;
  auto end = LineIterator::end();
  int running_sum = 0;
  std::regex pattern(
      "(?:mul\\((\\d{1,3}),(\\d{1,3})\\)|do\\(\\)|don\\'t\\(\\))");
  bool enabled = true;

  for (LineIterator it(input_path); it != end; ++it) {
    const auto &line = *it;
    auto words_begin = std::sregex_iterator(line.begin(), line.end(), pattern);
    auto words_end = std::sregex_iterator();
    for (std::sregex_iterator it = words_begin; it != words_end; ++it) {
      std::smatch match = *it;

      if (match.str() == "do()") {
        enabled = true;
        continue;
      } else if (match.str() == "don't()") {
        enabled = false;
        continue;
      }
      if (enabled) {
        running_sum += evaluate_mul_expression(match);
      }
    }
  }
  std::cout << "\nPart 2: " << running_sum << std::endl;
}
} // namespace aoc
