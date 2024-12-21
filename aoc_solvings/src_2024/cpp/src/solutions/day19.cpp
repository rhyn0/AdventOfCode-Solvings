#include "day19.hpp"
#include "utils/line_reader.hpp"
#include <deque>
#include <iostream>
#include <limits>
#include <sstream>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

typedef std::string INPUT;

// template <> struct std::hash<INPUT> {
//   size_t operator()(const INPUT &input) const {
//     return std::hash<std::string>()(input.first) ^ input.second;
//   }
// };

namespace day19 {
std::vector<std::string> parse_options(const std::string &line) {
  std::vector<std::string> options;
  std::stringstream reader(line);
  std::string option;
  // every option is separated by ', '
  // split on space and then trim off trailing comma if present
  while (std::getline(reader, option, ' ')) {
    size_t comma_idx = option.find(',');
    if (comma_idx != std::string::npos) {
      options.push_back(option.substr(0, comma_idx));
    } else {
      options.push_back(option);
    }
  }
  return options;
}
int min_towels_needed(const std::string &line,
                      const std::vector<std::string> &blocks,
                      std::unordered_map<std::string, int> &memo,
                      size_t idx = 0) {
  // if we have matched past the end of the string, then we have found a valid
  // solution.
  // INPUT pair = std::make_pair(line, idx);
  std::string pair = line.substr(idx);
  if (memo.contains(pair)) {
    return memo.at(pair);
  }
  if (idx == line.size()) {
    return 0;
  }
  int min_blocks = std::numeric_limits<int>::max();
  for (const auto &block : blocks) {
    // if the block is a substring at the start of the line, then we can use
    // that block
    if (line.find(block, idx) == idx) {
      int blocks_used =
          min_towels_needed(line, blocks, memo, idx + block.size());
      if (blocks_used != -1) {
        memo[pair] = blocks_used;
        min_blocks = std::min(min_blocks, blocks_used);
      }
    }
  }
  return min_blocks == std::numeric_limits<int>::max() ? -1 : min_blocks + 1;
}
long num_ways_to_make(const std::string &line,
                      const std::vector<std::string> &blocks,
                      std::unordered_map<INPUT, long> &memo, size_t idx = 0) {
  // if we have matched past the end of the string, then we have found a valid
  // solution.
  // INPUT pair = std::make_pair(line, idx);
  std::string pair = line.substr(idx);
  if (memo.contains(pair)) {
    return memo.at(pair);
  }
  if (idx == line.size()) {
    return 1;
  }
  long num_ways = 0;
  for (const auto &block : blocks) {
    // if the block is a substring at the start of the line, then we can use
    // that block
    if (line.find(block, idx) == idx) {
      num_ways += num_ways_to_make(line, blocks, memo, idx + block.size());
    }
  }
  memo[pair] = num_ways;
  return num_ways;
}
}; // namespace day19
namespace aoc {
using namespace day19;

void solve_day19_part1(const std::string &input_path) {
  LineIterator it(input_path);
  auto end = LineIterator::end();
  // track the min number of blocks used to build the line for valid lines.
  std::vector<int> min_blocks_used;
  std::vector<std::string> options = parse_options(*it);
  ++it;
  // blank line
  ++it;
  // now for every line, see if we can build that option
  std::unordered_map<INPUT, int> memo;
  for (; it != end; ++it) {
    int min_blocks = min_towels_needed(*it, options, memo);
    if (min_blocks != -1) {
      min_blocks_used.push_back(min_blocks);
    }
  }
  std::cout << "\nPart 1: " << min_blocks_used.size() << std::endl;
}
void solve_day19_part2(const std::string &input_path) {
  LineIterator it(input_path);
  auto end = LineIterator::end();
  // track the min number of blocks used to build the line for valid lines.
  long sum_num_ways = 0;
  std::vector<std::string> options = parse_options(*it);
  ++it;
  // blank line
  ++it;
  // now for every line, see if we can build that option
  std::unordered_map<INPUT, long> memo;
  for (; it != end; ++it) {
    sum_num_ways += num_ways_to_make(*it, options, memo);
  }
  std::cout << "\nPart 2: " << sum_num_ways << std::endl;
}
}; // namespace aoc
