#include "day05.hpp"
#include "utils/line_reader.hpp"
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

std::unordered_map<int, std::unordered_set<int>>
build_map(const std::string &input_path) {
  std::unordered_map<int, std::unordered_set<int>> map;
  auto end = LineIterator::end();
  for (LineIterator it(input_path); it != end; ++it) {
    if ((*it).empty()) {
      break;
    }
    std::stringstream reader(*it);
    std::string num;
    std::getline(reader, num, '|');
    int key = std::stoi(num);
    std::getline(reader, num, '|');
    int value = std::stoi(num);
    if (map.contains(key)) {
      map[key].insert(value);
    } else {
      std::unordered_set<int> set;
      set.insert(value);
      map[key] = set;
    }
  }

  return map;
}

/**
 * Return CSV numbers as a vector of integers
 */
void get_book_updates(const std::string &line, std::vector<int> &updates) {
  std::stringstream reader(line);
  std::string num;
  while (std::getline(reader, num, ',')) {
    updates.push_back(std::stoi(num));
  }
}

bool is_valid_book_update(
    const std::vector<int> &updates,
    const std::unordered_map<int, std::unordered_set<int>> &map) {
  std::unordered_set<int> invalid_updates;
  // iterate over updates in reverse
  for (int i = updates.size() - 1; i >= 0; i--) {
    int update = updates.at(i);
    if (invalid_updates.contains(update)) {
      // it is invalid and we can skip this line
      return false;
    }
    if (map.contains(update)) {
      for (int val : map.at(update)) {
        invalid_updates.insert(val);
      }
    }
  }
  return true;
}
int get_score_for_line(const std::vector<int> &updates) {
  // take the middle number of the line and add
  // if line is length 5, we need to take 3rd number
  int index = updates.size() / 2;
  return updates.at(index);
}

std::vector<int>
fixable_updates(const std::vector<int> &updates,
                const std::unordered_map<int, std::unordered_set<int>> &map) {

  // mapping of invalid numbers to the indices that added that requirement
  // TODO: add numbers to this array and then reorder the whole before it to be
  // valid.
  std::vector<int> fixed_updates;
  while (fixed_updates.size() < updates.size()) {
    int addition_idx = fixed_updates.size();
    // add the current number to the end of the vector
    fixed_updates.push_back(updates.at(addition_idx));
    while (!is_valid_book_update(fixed_updates, map)) {
      int temp = fixed_updates.at(addition_idx);
      fixed_updates.at(addition_idx) = fixed_updates.at(addition_idx - 1);
      fixed_updates.at(addition_idx - 1) = temp;
      addition_idx--;
    }
  }

  return fixed_updates;
}

namespace aoc {
void solve_day05_part1(const std::string &input_path) {
  std::unordered_map<int, std::unordered_set<int>> map = build_map(input_path);
  LineIterator it(input_path);
  auto end = LineIterator::end();
  int valid_book_updates = 0;
  // got to skip forward to the first line with updates
  while (it != end && !(*it).empty()) {
    ++it;
  }
  if (it == end) {
    std::cout << "FAILED TO FIND SECTION 2" << std::endl;
    return;
  }
  // skip this blank line
  ++it;
  for (; it != end; ++it) {
    std::string line = *it;
    if (line.empty()) {
      // hit end of section 2, trailing newline
      break;
    }

    std::vector<int> book_updates;
    get_book_updates(line, book_updates);
    if (is_valid_book_update(book_updates, map)) {
      valid_book_updates += get_score_for_line(book_updates);
    }
  }

  std::cout << "\nPart 1: " << valid_book_updates << std::endl;
}
void solve_day05_part2(const std::string &input_path) {
  std::unordered_map<int, std::unordered_set<int>> map = build_map(input_path);
  LineIterator it(input_path);
  auto end = LineIterator::end();
  int valid_book_updates = 0;
  // got to skip forward to the first line with updates
  while (it != end && !(*it).empty()) {
    ++it;
  }
  if (it == end) {
    std::cout << "FAILED TO FIND SECTION 2" << std::endl;
    return;
  }
  // skip this blank line
  ++it;
  for (; it != end; ++it) {
    std::string line = *it;
    if (line.empty()) {
      // hit end of section 2, trailing newline
      break;
    }

    std::vector<int> book_updates;
    get_book_updates(line, book_updates);
    if (!is_valid_book_update(book_updates, map)) {
      // fix the book updates to be correct
      std::vector<int> fixed_updates = fixable_updates(book_updates, map);

      valid_book_updates += get_score_for_line(fixed_updates);
    }
  }

  std::cout << "\nPart 2: " << valid_book_updates << std::endl;
}
} // namespace aoc
