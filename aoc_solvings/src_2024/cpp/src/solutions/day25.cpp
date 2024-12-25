#include "day25.hpp"
#include "utils/input_reader.hpp"
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

// LOCKs have 0 as first value. Other 5 values are 0-5
// KEYs have 1 as first value. Other 5 values are 0-5
typedef std::array<int, 6> LOCK_KEY;
inline std::ostream &operator<<(std::ostream &os, const LOCK_KEY &item) {
  return os << (item[0] == 0 ? "LOCK" : "KEY") << "(" << item[1] << ", "
            << item[2] << ", " << item[3] << ", " << item[4] << ", " << item[5]
            << ")";
}
namespace day25 {
LOCK_KEY parse_lock_key(const std::string &input) {
  LOCK_KEY item;
  item.fill(0);
  unsigned int row = 0, col = 0;
  for (char c : input) {
    if (row > 5) {
      // this will either be an empty row (for locks)
      // or the designator for a key, which is not valid data
      break;
    }
    if (row == 0 && col == 0) {
      // this is a lock, because it starts with #
      if (c == '#') {
        item[0] = 0;
      } else {
        // otherwise it is a key
        item[0] = 1;
      }
    }
    if (c == '\n') {
      row++;
      col = 0;
      continue;
    }
    if (row == 0) {
      col++;
      continue;
    }
    if (c == '#') {
      item[col + 1] += 1;
    }
    col++;
  }
  return item;
}
std::vector<LOCK_KEY> parse_input(const std::string &input_path) {
  std::vector<LOCK_KEY> keys;
  std::string content = FileReader::readFile(input_path);
  std::stringstream reader(content);
  std::string lock_key_item_str, subcontent;
  while (std::getline(reader, lock_key_item_str)) {
    if (lock_key_item_str.empty()) {
      keys.push_back(parse_lock_key(subcontent));
      subcontent.clear();
    } else {
      subcontent += lock_key_item_str;
      subcontent += '\n';
    }
  }
  if (!subcontent.empty()) {
    keys.push_back(parse_lock_key(subcontent));
  }
  return keys;
}
bool is_valid_combination(const LOCK_KEY &lock, const LOCK_KEY &key) {
  for (int i = 1; i < 6; i++) {
    if (lock[i] + key[i] > 5) {
      return false;
    }
  }
  return true;
}

}; // namespace day25
namespace aoc {
using namespace day25;

void solve_day25_part1(const std::string &input_path) {
  std::vector<LOCK_KEY> lock_keys = parse_input(input_path);
  std::vector<LOCK_KEY> locks, keys;
  int valid_combinations = 0;
  for (const LOCK_KEY &item : lock_keys) {
    if (item[0] == 0) {
      locks.push_back(item);
    } else {
      keys.push_back(item);
    }
  }
  for (const LOCK_KEY &lock : locks) {
    for (const LOCK_KEY &key : keys) {
      valid_combinations += is_valid_combination(lock, key);
    }
  }
  std::cout << "\nPart 1: " << valid_combinations << std::endl;
}
void solve_day25_part2(const std::string &input_path) {
  std::cout << "\nPart 2: " << input_path << std::endl;
}
}; // namespace aoc
