#include "day21.hpp"
#include "utils/line_reader.hpp"
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

template <> struct std::hash<std::pair<char, char>> {
  size_t operator()(const std::pair<char, char> &p) const {
    return hash<char>()(p.first) ^ (hash<char>()(p.second) << 1);
  }
};
typedef std::pair<std::string, int> CACHE_ENTRY;
template <> struct std::hash<CACHE_ENTRY> {
  size_t operator()(const CACHE_ENTRY &p) const {
    return hash<std::string>()(p.first) ^ (hash<int>()(p.second) << 1);
  }
};
namespace day21 {
// mapping of pairs of shortest distances for the numeric keypad
// but no reverses - such as I have from A to any number, but not from 0 to A
// because the reverse is easy to calculate given the normal
/**
 * Reminder of the layout of the numeric keypad
 * +---+---+---+
 * | 7 | 8 | 9 |
 * +---+---+---+
 * | 4 | 5 | 6 |
 * +---+---+---+
 * | 1 | 2 | 3 |
 * +---+---+---+
 *     | 0 | A |
 *     +---+---+
 * Note that the missing bottom left is intentional
 * There are some reverse entries in the mapping, to avoid the missing hole
 */
const std::unordered_map<std::pair<char, char>, std::string> keypadMapping = {
    {{'A', '0'}, "<A"},
    {{'A', '1'}, "^<<A"},
    {{'1', 'A'}, ">>vA"},
    {{'A', '2'}, "<^A"},
    {{'2', 'A'}, "v>A"},
    {{'A', '3'}, "^A"},
    {{'A', '4'}, "^^<<A"},
    {{'4', 'A'}, ">>vvA"},
    {{'A', '5'}, "<^^A"},
    {{'5', 'A'}, "vv>A"},
    {{'A', '6'}, "^^A"},
    {{'A', '7'}, "^^^<<A"},
    {{'7', 'A'}, ">>vvvA"},
    {{'A', '8'}, "<^^^A"},
    {{'8', 'A'}, "vvv>A"},
    {{'A', '9'}, "^^^A"},
    {{'0', '1'}, "^<A"},
    {{'1', '0'}, ">vA"},
    {{'0', '2'}, "^A"},
    {{'0', '3'}, "^>A"},
    {{'3', '0'}, "<vA"},
    {{'0', '4'}, "^<^A"},
    {{'4', '0'}, ">vvA"},
    {{'0', '5'}, "^^A"},
    {{'0', '6'}, "^^>A"},
    {{'6', '0'}, "<vvA"},
    {{'0', '7'}, "^^^<A"},
    {{'7', '0'}, ">vvvA"},
    {{'0', '8'}, "^^^A"},
    {{'0', '9'}, "^^^>A"},
    {{'9', '0'}, "<vvvA"},
    {{'1', '2'}, ">A"},
    {{'1', '3'}, ">>A"},
    {{'1', '4'}, "^A"},
    {{'1', '5'}, "^>A"},
    {{'5', '1'}, "<vA"},
    {{'1', '6'}, "^>>A"},
    {{'6', '1'}, "<<vA"},
    {{'1', '7'}, "^^A"},
    {{'1', '8'}, "^^>A"},
    {{'8', '1'}, "<vvA"},
    {{'1', '9'}, "^^>>A"},
    {{'9', '1'}, "<<vvA"},
    {{'2', '3'}, ">A"},
    {{'2', '4'}, "<^A"},
    {{'4', '2'}, "v>A"},
    {{'2', '5'}, "^A"},
    {{'2', '6'}, "^>A"},
    {{'6', '2'}, "<vA"},
    {{'2', '7'}, "<^^A"},
    {{'7', '2'}, "vv>A"},
    {{'2', '8'}, "^^A"},
    {{'2', '9'}, "^^>A"},
    {{'9', '2'}, "<vvA"},
    {{'3', '4'}, "<<^A"},
    {{'4', '3'}, "v>>A"},
    {{'3', '5'}, "<^A"},
    {{'5', '3'}, "v>A"},
    {{'3', '6'}, "^A"},
    {{'3', '7'}, "<<^^A"},
    {{'7', '3'}, "vv>>A"},
    {{'3', '8'}, "<^^A"},
    {{'8', '3'}, "vv>A"},
    {{'3', '9'}, "^^A"},
    {{'4', '5'}, ">A"},
    {{'4', '6'}, ">>A"},
    {{'4', '7'}, "^A"},
    {{'4', '8'}, "^>A"},
    {{'8', '4'}, "<vA"},
    {{'4', '9'}, "^>>A"},
    {{'9', '4'}, "<<vA"},
    {{'5', '6'}, ">A"},
    {{'5', '7'}, "<^A"},
    {{'7', '5'}, "v>A"},
    {{'5', '8'}, "^A"},
    {{'5', '9'}, "^>A"},
    {{'9', '5'}, "<vA"},
    {{'6', '7'}, "<<^A"},
    {{'7', '6'}, "v>>A"},
    {{'6', '8'}, "<^A"},
    {{'8', '6'}, "v>A"},
    {{'6', '9'}, "^A"},
    {{'7', '8'}, ">A"},
    {{'7', '9'}, ">>A"},
    {{'8', '9'}, ">A"},
    // now for directional keypad
    /**
     * Reminder of the layout of the directional keypad
     *     +---+---+
     *     | ^ | A |
     * +---+---+---+
     * | < | v | > |
     * +---+---+---+
     * Note that the missing top right is intentional
     * There are some reverse entries in the mapping, to avoid the missing
     hole
     */
    {{'A', '^'}, "<A"},
    {{'A', '>'}, "vA"},
    {{'A', 'v'}, "<vA"},
    {{'v', 'A'}, "^>A"},
    {{'A', '<'}, "v<<A"},
    {{'<', 'A'}, ">>^A"},
    {{'^', '<'}, "v<A"},
    {{'<', '^'}, ">^A"},
    {{'^', 'v'}, "vA"},
    {{'^', '>'}, "v>A"},
    {{'>', '^'}, "<^A"},
    {{'<', 'v'}, ">A"},
    {{'<', '>'}, ">>A"},
    {{'v', '>'}, ">A"},

};

std::string reverse_directions(const std::string &directions) {
  std::string reversed;
  for (char c : directions) {
    switch (c) {
    case '^':
      reversed += 'v';
      break;
    case 'v':
      reversed += '^';
      break;
    case '<':
      reversed += '>';
      break;
    case '>':
      reversed += '<';
      break;
    default:
      // if it's A, it stays A
      reversed += c;
    }
  }
  return reversed;
}
std::string single_step_conversion(char previous, char current) {
  std::pair<char, char> pair = {previous, current};
  if (current == previous) {
    return "A";
  } else if (keypadMapping.contains(pair)) {
    return keypadMapping.at(pair);
  } else {
    return reverse_directions(keypadMapping.at({current, previous}));
  }
}
std::string convert_input_to_directions(const std::string &code) {
  std::string converted;
  // arm starts at A always
  char previous = 'A';
  for (char c : code) {
    converted += single_step_conversion(previous, c);
    previous = c;
  }
  return converted;
}
// given the numeric code to input, convert to the directions I need to input
std::string get_my_directions(const std::string &code) {
  std::string first_robot = convert_input_to_directions(code);
  std::string second_robot = convert_input_to_directions(first_robot);
  return convert_input_to_directions(second_robot);
}

std::unordered_map<CACHE_ENTRY, long> cache = {};
long sequence_length(const std::string &code, int robots_left) {
  CACHE_ENTRY entry = {code, robots_left};
  if (cache.contains(entry)) {
    return cache.at(entry);
  }
  long length = 0;
  if (robots_left == 0) {
    length = code.length();
  } else {
    char previous = 'A';
    for (char c : code) {
      long char_conversion_len;
      std::string new_code = single_step_conversion(previous, c);
      if (c == previous) {
        char_conversion_len = 1;
      } else {
        char_conversion_len = sequence_length(new_code, robots_left - 1);
      };
      previous = c;
      length += char_conversion_len;
    }
  }
  cache[entry] = length;
  return length;
}

// return the numeric value of the string
int get_numeric_value(const std::string &code) {
  // last character is A always, so just trim that off
  return std::stoi(code.substr(0, code.length() - 1));
}

}; // namespace day21
namespace aoc {
using namespace day21;

void solve_day21_part1(const std::string &input_path) {
  LineIterator it(input_path);
  long complexity_sum = 0;
  while (!(*it).empty()) {
    int value = get_numeric_value(*it);
    std::string directions = get_my_directions(*it);
    complexity_sum += (value * directions.length());
    ++it;
  }
  std::cout << "\nPart 1: " << complexity_sum << std::endl;
}
void solve_day21_part2(const std::string &input_path) {
  LineIterator it(input_path);
  long complexity_sum = 0;
  while (!(*it).empty()) {
    int value = get_numeric_value(*it);
    std::string first_directional_directions = convert_input_to_directions(*it);
    long final_length = sequence_length(first_directional_directions, 25);
    complexity_sum += (value * final_length);
    ++it;
  }
  std::cout << "\nPart 2: " << complexity_sum << std::endl;
}
}; // namespace aoc
