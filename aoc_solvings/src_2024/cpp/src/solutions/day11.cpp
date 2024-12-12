#include "day11.hpp"
#include "utils/line_reader.hpp"
#include <algorithm>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

namespace day11 {
class FloatingStone {
public:
  long replicas;
  long value;
  FloatingStone(long value) : replicas(1), value(value) {}
  FloatingStone(long value, long replicas) : replicas(replicas), value(value) {}
  long get_value() const { return value; }
  std::string get_value_string() const { return std::to_string(value); }
  /**
   * If the stone value is 0, change it to 1.
   * If the number of digits in the stone value is even, split into two stones
   * of left digit and right digit. Otherwise multiply the stone value by 2024.
   * @return The vector of stones produced by the blink
   */
  std::vector<FloatingStone> blink() {
    std::vector<FloatingStone> result;
    if (value == 0) {
      result.push_back(FloatingStone(1, replicas));
      return result;
    }
    std::string value_string = this->get_value_string();
    if (value_string.length() % 2 == 0) {
      // split the value number into left digits and right digits
      result.push_back(FloatingStone(
          std::stol(value_string.substr(0, value_string.length() / 2)),
          replicas));
      result.push_back(FloatingStone(
          std::stol(value_string.substr(value_string.length() / 2)), replicas));
      return result;
    }
    result.push_back(FloatingStone(value * 2024, replicas));
    return result;
  }

  bool operator==(const FloatingStone &other) const {
    return value == other.value && replicas == other.replicas;
  }
  void increase_replicas(long amount) { replicas += amount; }
};

// Add operator<< overload for FloatingStone
inline std::ostream &operator<<(std::ostream &os, const FloatingStone &stone) {
  return os << "FloatingStone(" << stone.get_value() << ", " << stone.replicas
            << ")";
}

std::vector<FloatingStone> parse_line(const std::string &input_path) {
  std::vector<FloatingStone> stones;
  LineIterator it(input_path);
  std::string line = *it;
  std::stringstream reader(line);
  std::string num;
  while (std::getline(reader, num, ' ')) {
    stones.push_back(FloatingStone(std::stol(num)));
  }
  return stones;
}

std::vector<FloatingStone> blink_stones(std::vector<FloatingStone> &stones) {
  std::vector<FloatingStone> new_stones;
  for (FloatingStone &stone : stones) {
    std::vector<FloatingStone> this_blink = stone.blink();
    for (FloatingStone &new_stone : this_blink) {
      auto it = std::find(new_stones.begin(), new_stones.end(), new_stone);
      if (it != new_stones.end()) {
        it->increase_replicas(new_stone.replicas);
      } else {
        new_stones.push_back(new_stone);
      }
    }
  }
  return new_stones;
}

}; // namespace day11
namespace aoc {
using namespace day11;

void solve_day11_part1(const std::string &input_path) {
  std::vector<FloatingStone> stones = parse_line(input_path);
  for (int i = 0; i < 25; i++) {
    stones = blink_stones(stones);
  }
  long total_replicas = 0;
  for (const FloatingStone &stone : stones) {
    total_replicas += stone.replicas;
  }
  std::cout << "\nPart 1: " << total_replicas << std::endl;
}
void solve_day11_part2(const std::string &input_path) {
  std::vector<FloatingStone> stones = parse_line(input_path);
  for (int i = 0; i < 75; i++) {
    stones = blink_stones(stones);
  }
  long total_replicas = 0;
  for (const FloatingStone &stone : stones) {
    total_replicas += stone.replicas;
  }
  std::cout << "\nPart 2: " << total_replicas << std::endl;
}
}; // namespace aoc
