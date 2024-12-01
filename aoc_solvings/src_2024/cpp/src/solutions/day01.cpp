#include "day01.hpp"
#include "utils/input_reader.hpp"
#include <algorithm>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

// hard code the delimiter as a space
static const char delimiter = ' ';

int get_number(std::stringstream &line) {
  std::string buffer;
  while (buffer.empty()) {
    if (!std::getline(line, buffer, delimiter)) {
      throw std::runtime_error("Invalid input file, expected two numbers per "
                               "line separated by spaces.");
    }
  }
  return std::stoi(buffer);
}

void parse_line(std::vector<int> &first, std::vector<int> &second,
                const std::string &line_buffer) {
  std::stringstream line(line_buffer);
  first.push_back(get_number(std::ref(line)));
  second.push_back(get_number(std::ref(line)));
}

void build_number_lists(std::vector<int> &first, std::vector<int> &second,
                        const std::string &file_path) {
  std::string content = FileReader::readFile(file_path);
  std::stringstream reader(content);
  std::string line_buffer;
  // for every line, separate the numbers based on whitespace (input shows 3
  // spaces)
  while (std::getline(reader, line_buffer)) {
    if (!line_buffer.empty()) {
      parse_line(first, second, line_buffer);
    }
  }
}

namespace aoc {
/**
 * @brief Solve for Part 1 Day 1, finding the absolute difference between two
 * sets of numbers
 *
 * Given a file with two numbers per line, separated by spaces, gather the
 * numbers into their respective columns. Then sort and return the sum of the
 * absolute difference between the smallest of each column, the second smallest
 * and so on.
 *
 * @param input_path - file path to data to solve with
 */
void solve_day01_part1(const std::string &input_path) {
  // Read input file
  std::vector<int> first_list, second_list;
  long total_distance = 0;

  build_number_lists(first_list, second_list, input_path);
  // sort the two lists of numbers
  std::sort(first_list.begin(), first_list.end());
  std::sort(second_list.begin(), second_list.end());

  for (size_t idx = 0; idx < first_list.size(); idx++) {
    total_distance += std::abs(first_list.at(idx) - second_list.at(idx));
  }

  std::cout << "\nPart 1: " << total_distance << std::endl;
}
/**
 * @brief Solve for Part 2 Day 1, finding the similarity between two lists of
 * numbers.
 *
 * Given a file with two numbers per line, separated by spaces, gather the
 * numbers into respective columns. Then for each number in first column, count
 * the occurrences of that number in the second column. Return the number of
 * ocurrences multiplied by the value of the number as the similarity score for
 * that particular number. Output the total similarity score.
 *
 * @param input_path - file path to data to solve with
 */
void solve_day01_part2(const std::string &input_path) {
  // Read input file
  std::vector<int> first_list, second_list;
  long similarity_score = 0;
  std::unordered_map<int, int> counts;
  build_number_lists(first_list, second_list, input_path);

  // build the counts of every number in the second list
  for (auto &&val : second_list) {
    if (!counts.contains(val)) {
      counts[val] = 1;
    } else {
      counts[val] = counts[val] + 1;
    }
  }

  // equation for similarity score is the number times the number of times
  // it appears in the second list
  for (auto &&val : first_list) {
    similarity_score += val * counts[val];
  }

  std::cout << "\nPart 2: " << similarity_score << std::endl;
}
} // namespace aoc
