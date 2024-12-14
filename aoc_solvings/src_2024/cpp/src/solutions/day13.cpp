#include "day13.hpp"
#include "utils/input_reader.hpp"
#include <_types/_uint64_t.h>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <regex>
#include <sstream>
#include <string>
#include <vector>
// hehhe
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <string>

namespace day13 {
// did some linear algebra for this by hand
// A_x\alpha + B_x\beta = P_x
// A_y\alpha + B_y\beta = P_y
// 0 \le \alpha, \beta \le 100
// 3 \alpha + \beta = \text{cost}
// \alpha = \frac {P_x - B_x\beta}{A_x}
// \beta = \frac {A_xP_y - A_yP_x}{(A_xB_y - A_yB_x)}

struct XYMove {
  uint64_t x, y;
};
inline uint64_t solve_crane_lin_alg(const XYMove &A, const XYMove &B,
                                    const XYMove &P, const bool &part2) {
  // make sure that alpha and beta are whole integers
  const auto [b_presses, b_valid] =
      std::lldiv((A.x * P.y - A.y * P.x), (A.x * B.y - A.y * B.x));
  if (b_valid != 0) {
    return 0;
  }
  const auto [a_presses, a_valid] = std::lldiv((P.x - B.x * b_presses), A.x);
  if (a_valid != 0) {
    return 0;
  }
  if (!part2 && (a_presses > 100 || b_presses > 100)) {
    return 0;
  }
  return 3 * a_presses + b_presses;
}

std::regex button_regex("X\\+(\\d+), Y\\+(\\d+)");
std::regex prize_regex("X=(\\d+), Y=(\\d+)");

std::vector<unsigned long long>
parse_crane_problem(const std::vector<std::string> &content) {
  std::vector<unsigned long long> crane_problem;
  std::smatch match;

  // there are two buttons and one prize
  if (std::regex_search(content[0], match, button_regex)) {
    crane_problem.push_back(std::stoull(match[1]));
    crane_problem.push_back(std::stoull(match[2]));
  }
  if (std::regex_search(content[1], match, button_regex)) {
    crane_problem.push_back(std::stoull(match[1]));
    crane_problem.push_back(std::stoull(match[2]));
  }
  if (std::regex_search(content[2], match, prize_regex)) {
    crane_problem.push_back(std::stoull(match[1]));
    crane_problem.push_back(std::stoull(match[2]));
  }

  return crane_problem;
}
std::vector<std::vector<unsigned long long>>
get_cranes(const std::string &input_path) {
  std::string content = FileReader::readFile(input_path);
  std::stringstream reader(content);
  std::string line;
  std::vector<std::string> section;
  std::vector<std::vector<unsigned long long>> crane_problems;

  while (std::getline(reader, line)) {
    if (line.empty()) {
      if (!section.empty()) {
        crane_problems.push_back(parse_crane_problem(section));
        section.clear();
      }
    } else {
      section.push_back(line);
    }
  }
  // Don't forget the last section if it doesn't end with a blank line
  if (!section.empty()) {
    crane_problems.push_back(parse_crane_problem(section));
  }
  return crane_problems;
}
}; // namespace day13
namespace aoc {
using namespace day13;

void solve_day13_part1(const std::string &input_path) {
  auto crane_problems = get_cranes(input_path);
  uint64_t total_cost = 0;
  XYMove A, B, P;
  for (auto crane_problem : crane_problems) {
    A.x = crane_problem[0];
    A.y = crane_problem[1];
    B.x = crane_problem[2];
    B.y = crane_problem[3];
    P.x = crane_problem[4];
    P.y = crane_problem[5];
    total_cost += solve_crane_lin_alg(A, B, P, false);
  }
  std::cout << "\nPart 1: " << total_cost << std::endl;
}
const uint64_t conversion_incr = 10000000000000;
void solve_day13_part2(const std::string &input_path) {
  auto crane_problems = get_cranes(input_path);

  uint64_t total_cost = 0;
  XYMove A, B, P;
  for (auto crane_problem : crane_problems) {
    A.x = crane_problem[0];
    A.y = crane_problem[1];
    B.x = crane_problem[2];
    B.y = crane_problem[3];
    P.x = crane_problem[4] + conversion_incr;
    P.y = crane_problem[5] + conversion_incr;
    total_cost += solve_crane_lin_alg(A, B, P, true);
  }
  std::cout << "\nPart 2: " << total_cost << std::endl;
}
}; // namespace aoc
