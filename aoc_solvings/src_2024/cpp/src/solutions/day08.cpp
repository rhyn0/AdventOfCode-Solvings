#include "day08.hpp"
#include "utils/input_reader.hpp"
#include <iostream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

class Position {
public:
  int row;
  int col;
  Position(int r, int c) : row(r), col(c) {}
  Position() : row(0), col(0) {}
  Position operator-(const Position &other) const {
    return Position(row - other.row, col - other.col);
  };
  Position operator+(const Position &other) const {
    return Position(row + other.row, col + other.col);
  };
  bool operator==(const Position &other) const {
    return row == other.row && col == other.col;
  }
  bool is_contained_in(int max_row, int max_col) const {
    return row >= 0 && row < max_row && col >= 0 && col < max_col;
  }
  size_t hash() const {
    return std::hash<int>{}(row) ^ (std::hash<int>{}(col) << 1);
  }
};
namespace std {
template <> struct hash<Position> {
  size_t operator()(const Position &pos) const { return pos.hash(); }
};
} // namespace std

/**
 * Return the groups of antennas and their positions.
 */
std::unordered_map<char, std::vector<Position>>
parse_grid(const std::string &content) {
  std::unordered_map<char, std::vector<Position>> grid = {};
  int row = 0;
  int col = 0;
  for (char c : content) {
    if (c == '\n') {
      row += 1;
      col = 0;
      continue;
    } else if (c != '.') {
      if (grid.find(c) == grid.end()) {
        grid[c] = std::vector<Position>();
      }
      grid[c].push_back(Position(row, col));
    }
    col += 1;
  }

  return grid;
}

std::pair<int, int> get_grid_bounds(const std::string &content) {
  int row = 0;
  int max_col = 0;
  int col = 0;
  for (char c : content) {
    if (c == '\n') {
      row += 1;
      col = 0;
    } else {
      col += 1;
      max_col = std::max(max_col, col);
    }
  }
  return std::make_pair(row, max_col);
}

namespace aoc {
void solve_day08_part1(const std::string &input_path) {
  std::string content = FileReader::readFile(input_path);
  std::unordered_map<char, std::vector<Position>> antennas =
      parse_grid(content);
  int num_rows, num_cols;
  std::tie(num_rows, num_cols) = get_grid_bounds(content);
  // for every grouping of antennas
  // make a pair and compute the antinode locations
  std::unordered_set<Position> antinodes = std::unordered_set<Position>();
  for (auto &entry : antennas) {
    std::vector<Position> &antennae = entry.second;
    for (size_t i = 0; i < antennae.size(); i++) {
      for (size_t j = i + 1; j < antennae.size(); j++) {
        Position delta = antennae[i] - antennae[j];
        Position antinode_1 = antennae[i] + delta;
        Position antinode_2 = antennae[j] - delta;
        if (antinode_1.is_contained_in(num_rows, num_cols)) {
          antinodes.insert(antinode_1);
        }
        if (antinode_2.is_contained_in(num_rows, num_cols)) {
          antinodes.insert(antinode_2);
        }
      }
    }
  }
  std::cout << "\nPart 1: " << antinodes.size() << std::endl;
}
void solve_day08_part2(const std::string &input_path) {
  std::string content = FileReader::readFile(input_path);
  std::unordered_map<char, std::vector<Position>> antennas =
      parse_grid(content);
  int num_rows, num_cols;
  std::tie(num_rows, num_cols) = get_grid_bounds(content);
  // for every grouping of antennas
  // make a pair and compute the antinode locations
  std::unordered_set<Position> antinodes = std::unordered_set<Position>();
  for (auto &entry : antennas) {
    std::vector<Position> &antennae = entry.second;
    for (size_t i = 0; i < antennae.size(); i++) {
      antinodes.insert(antennae[i]);
      for (size_t j = i + 1; j < antennae.size(); j++) {
        Position delta = antennae[i] - antennae[j];
        // calculate all the antinodes going forwards
        Position curr = antennae[i] + delta;
        while (curr.is_contained_in(num_rows, num_cols)) {
          antinodes.insert(curr);
          curr = curr + delta;
        }
        // then repeat for backwards
        curr = antennae[j];
        while (curr.is_contained_in(num_rows, num_cols)) {
          antinodes.insert(curr);
          curr = curr - delta;
        }
      }
    }
  }
  std::cout << "\nPart 2: " << antinodes.size() << std::endl;
}
} // namespace aoc
