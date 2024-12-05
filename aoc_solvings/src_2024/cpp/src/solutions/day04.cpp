#include "day02.hpp"
#include "utils/input_reader.hpp"
#include <iostream>
#include <string>
#include <vector>

int scan_directions(std::vector<std::vector<char>> &grid, size_t startRow,
                    size_t startCol, std::string target) {
  int matches = 0;
  // Check if grid is empty
  if (grid.empty() || grid[0].empty()) {
    return 0;
  }

  size_t rows = grid.size();
  size_t cols = grid[0].size();
  size_t target_length = target.size();

  // Define direction arrays for 8 possible directions
  std::vector<int> dx = {-1, -1, -1, 0, 0, 1, 1, 1};
  std::vector<int> dy = {-1, 0, 1, -1, 1, -1, 0, 1};

  // Check if starting position is valid
  if (startRow >= rows || startCol >= cols) {
    return 0;
  }

  for (int dir = 0; dir < 8; dir++) {
    std::string str;
    int currRow = static_cast<int>(startRow);
    int currCol = static_cast<int>(startCol);

    // Build string of specified length in current direction
    for (size_t i = 0; i < target_length; i++) {
      // std::cout << "trying character " << i << "while looking for XMAS" <<
      // std::endl;
      if (currRow >= 0 && currRow < static_cast<int>(rows) && currCol >= 0 &&
          currCol < static_cast<int>(cols)) {
        str.push_back(grid[currRow][currCol]);

        currRow += dx[dir];
        currCol += dy[dir];
      } else {
        break;
      }
    }

    if (str.length() == target_length && str == target) {
      matches += 1;
      str.clear();
    }
  }

  return matches;
}

std::pair<std::string, std::string>
get_x_strings(std::vector<std::vector<char>> &grid, size_t startRow,
              size_t startCol) {
  std::string first, second;
  first.push_back(grid[startRow][startCol]);
  first.push_back(grid[startRow + 1][startCol + 1]);
  first.push_back(grid[startRow + 2][startCol + 2]);

  second.push_back(grid[startRow][startCol + 2]);
  second.push_back(grid[startRow + 1][startCol + 1]);
  second.push_back(grid[startRow + 2][startCol]);
  return std::make_pair(first, second);
}

std::vector<std::vector<char>> read_grid(const std::string &filepath) {
  std::string content = FileReader::readFile(filepath);
  std::vector<std::vector<char>> grid = {{}};
  for (char c : content) {
    if (c == '\n') {
      grid.push_back(std::vector<char>());
    } else {
      grid[grid.size() - 1].push_back(c);
    }
  }
  if (grid[grid.size() - 1].empty()) {
    grid.pop_back();
  }
  return grid;
}

namespace aoc {
void solve_day04_part1(const std::string &input_path) {
  int total_matches = 0;
  std::vector<std::vector<char>> grid = read_grid(input_path);

  size_t rows = grid.size();
  size_t cols = grid[0].size();

  // Check if grid is empty
  if (grid.empty() || grid[0].empty()) {
    return;
  }
  for (size_t row = 0; row < rows; row++) {
    for (size_t col = 0; col < cols; col++) {
      if (grid[row][col] == 'X') {
        total_matches += scan_directions(grid, row, col, "XMAS");
      }
    }
  }

  std::cout << "\nPart 1: " << total_matches << std::endl;
}
void solve_day04_part2(const std::string &input_path) {
  int total_matches = 0;
  std::vector<std::vector<char>> grid = read_grid(input_path);

  size_t rows = grid.size();
  size_t cols = grid[0].size();

  // Check if grid is empty
  if (grid.empty() || grid[0].empty()) {
    return;
  }
  for (size_t row = 0; row < rows - 2; row++) {
    for (size_t col = 0; col < cols - 2; col++) {
      std::string left_right, right_left;
      std::tie(left_right, right_left) = get_x_strings(grid, row, col);
      if ((left_right == "MAS" && right_left == "MAS") ||
          (left_right == "MAS" && right_left == "SAM") ||
          (left_right == "SAM" && right_left == "MAS") ||
          (left_right == "SAM" && right_left == "SAM")) {
        total_matches += 1;
      }
    }
  }
  std::cout << "\nPart 2: " << total_matches << std::endl;
}
} // namespace aoc
