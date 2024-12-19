#include "day17.hpp"
#include "utils/input_reader.hpp"
#include <__fwd/get.h>
#include <algorithm>
#include <cmath>
#include <cstddef>
#include <iostream>
#include <regex>
#include <stdexcept>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>

namespace day17 {
std::regex number_regex("\\d+");
long parse_register(const std::string &line) {
  std::smatch match;
  std::regex_search(line, match, number_regex);
  return std::stol(match[0]);
}
std::vector<int> parse_instructions(const std::string &section) {
  std::vector<int> instructions;
  std::smatch match;
  std::string curr = section;
  // line is a csv of numbers. while there are numbers in the line, add them to
  while (std::regex_search(curr, match, number_regex)) {
    instructions.push_back(std::stol(match[0]));
    curr = curr.substr(match.position() + 1);
  }
  return instructions;
}
typedef std::tuple<long, long, long> REG;
REG get_registers(const std::string &content) {
  int register_a, register_b, register_c;
  size_t start = 0;
  register_a = parse_register(content.substr(0, content.find("\n", start)));
  start = content.find("\n", start) + 1;
  register_b = parse_register(content.substr(start, content.find("\n", start)));
  start = content.find("\n", start) + 1;
  register_c = parse_register(content.substr(start, content.find("\n", start)));
  return std::make_tuple(register_a, register_b, register_c);
}

long combo_operator(REG registers, int orig_operand) {
  switch (orig_operand) {
  case 0:
  case 1:
  case 2:
  case 3:
    return orig_operand;
  case 4:
    return std::get<0>(registers);
  case 5:
    return std::get<1>(registers);
  case 6:
    return std::get<2>(registers);
  default:
    throw std::runtime_error("INVALID COMBO OPERAND");
  }
}
// operations defined for our computer
REG adv(REG registers, int operand) {
  long combo = combo_operator(registers, operand);
  long new_a = std::get<0>(registers) >> combo;

  return std::make_tuple(new_a, std::get<1>(registers), std::get<2>(registers));
}
REG bxl(REG registers, int operand) {
  long new_b = std::get<1>(registers) ^ operand;
  return std::make_tuple(std::get<0>(registers), new_b, std::get<2>(registers));
}
REG bst(REG registers, int operand) {
  long new_b = combo_operator(registers, operand) & 0b111;
  return std::make_tuple(std::get<0>(registers), new_b, std::get<2>(registers));
}
REG bxc(REG registers) {
  long new_b = std::get<1>(registers) ^ std::get<2>(registers);
  return std::make_tuple(std::get<0>(registers), new_b, std::get<2>(registers));
};
REG bdv(REG registers, int operand) {
  long combo = combo_operator(registers, operand);
  long new_b = std::get<0>(registers) >> combo;

  return std::make_tuple(std::get<0>(registers), new_b, std::get<2>(registers));
}
REG cdv(REG registers, int operand) {
  long combo = combo_operator(registers, operand);
  long new_c = std::get<0>(registers) >> combo;

  return std::make_tuple(std::get<0>(registers), std::get<1>(registers), new_c);
}
std::vector<int> run_program(REG registers, std::vector<int> instructions) {
  std::vector<int> output;
  for (size_t i = 0; i < instructions.size();) {
    int opcode = instructions[i];
    int operand = instructions[i + 1];
    switch (opcode) {
    case 0:
      registers = adv(registers, operand);
      break;
    case 1:
      registers = bxl(registers, operand);
      break;
    case 2:
      registers = bst(registers, operand);
      break;
    case 3:
      // if A register is 0, do nothing
      if (std::get<0>(registers) == 0) {
        break;
      }
      // otherwise jump instruction pointer to literal operand
      // skip increasing the index of the instruction pointer
      i = operand;
      continue;
    case 4:
      registers = bxc(registers);
      break;
    case 5:
      output.push_back(combo_operator(registers, operand) & 0b111);
      break;
    case 6:
      registers = bdv(registers, operand);
      break;
    case 7:
      registers = cdv(registers, operand);
      break;
    default:
      throw std::runtime_error("INVALID OPCODE");
    }
    i += 2;
  }
  return output;
}

void search_a(std::vector<int> instructions, size_t target_idx_from_end,
              long long starting_a, std::vector<long long> &results) {
  int target_val = instructions[instructions.size() - target_idx_from_end];
  // play with modifying the bottom 3 bits of `starting_a`
  for (long a = 0; a < 8; a++) {
    std::vector<int> returned_instructions =
        run_program(std::make_tuple(starting_a + a, 0, 0), instructions);
    if (returned_instructions[0] == target_val) {
      // we have a good starting_a, but have to go to next level
      if (target_idx_from_end == instructions.size()) {
        results.push_back(starting_a + a);
      } else if (target_idx_from_end < instructions.size()) {
        // recurse to next solution
        search_a(instructions, target_idx_from_end + 1, (starting_a + a) * 8,
                 results);
      }
    }
  }
}
}; // namespace day17
namespace aoc {
using namespace day17;

void solve_day17_part1(const std::string &input_path) {
  std::string content = FileReader::readFile(input_path);
  std::tuple<int, int, int> registers = get_registers(content);
  std::vector<int> instructions =
      parse_instructions(content.substr(content.find("\n\n") + 2));
  std::vector<int> output = run_program(registers, instructions);
  std::cout << "\nPart 1: ";
  for (int val : output) {
    std::cout << val << ",";
  }
  std::cout << std::endl;
}
void solve_day17_part2(const std::string &input_path) {
  std::string content = FileReader::readFile(input_path);
  std::vector<int> instructions =
      parse_instructions(content.substr(content.find("\n\n") + 2));
  std::vector<long long> results;

  search_a(instructions, 1, 0, results);
  long answer = *std::min_element(results.begin(), results.end());

  std::cout << "\nPart 1: " << answer << std::endl;
}
}; // namespace aoc
