#include "day06.hpp"
#include "utils/input_reader.hpp"
#include <algorithm>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

// ROW, COL order
typedef std::pair<int, int> POS;

// Custom hash function for POS (std::pair)
namespace std {
template <> struct hash<POS> {
  size_t operator()(const POS &p) const {
    return hash<int>()(p.first) ^ (hash<int>()(p.second) << 1);
  }
};
} // namespace std

std::vector<std::string> get_board_rows(const std::string &input_path) {
  // split the board on newlines
  std::string board_content = FileReader::readFile(input_path);
  std::vector<std::string> board_lines;
  std::stringstream reader(board_content);
  std::string line;
  while (std::getline(reader, line)) {
    board_lines.push_back(line);
  }
  return board_lines;
}

/**
 * Board is a 2D grid made up of '.', '#', and '^'.
 * Periods represent empty space.
 * Octothorpe is an obstacle.
 * Caret is player starting position, representing looking upwards.
 *
 * Return vector of just obstacles.
 */
std::vector<POS> get_obstacles(const std::vector<std::string> &board_content) {
  std::vector<POS> obstacles;
  for (size_t i = 0; i < board_content.size(); i++) {
    for (size_t j = 0; j < board_content[i].size(); j++) {
      if (board_content[i][j] == '#') {
        obstacles.push_back(std::make_pair(i, j));
      }
    }
  }
  return obstacles;
}

POS get_starting_position(const std::vector<std::string> &board_content) {
  for (size_t i = 0; i < board_content.size(); i++) {
    for (size_t j = 0; j < board_content[i].size(); j++) {
      if (board_content[i][j] == '^') {
        return std::make_pair(i, j);
      }
    }
  }
  return std::make_pair(-1, -1);
}

bool on_board(const POS position, int num_rows, int num_cols) {
  return position.first >= 0 && position.first < num_rows &&
         position.second >= 0 && position.second < num_cols;
}

void visualize_board_state(const std::vector<POS> &obstacles,
                           const std::unordered_set<POS> &visited, int num_rows,
                           int num_cols) {
  for (int i = 0; i < num_rows; i++) {
    for (int j = 0; j < num_cols; j++) {
      if (std::find(obstacles.begin(), obstacles.end(), std::make_pair(i, j)) !=
          obstacles.end()) {
        std::cout << "#";
      } else if (std::find(visited.begin(), visited.end(),
                           std::make_pair(i, j)) != visited.end()) {
        std::cout << "X";
      } else {
        std::cout << ".";
      }
    }
    std::cout << std::endl;
  }
}

const static std::vector<std::pair<int, int>> directions = {
    {-1, 0}, // up
    {0, 1},  // right
    {1, 0},  // down
    {0, -1}  // left
};

POS next_position(const POS position, int direction_idx) {
  return std::make_pair(position.first + directions[direction_idx].first,
                        position.second + directions[direction_idx].second);
}

std::unordered_set<POS> get_visited_positions(const std::vector<POS> &obstacles,
                                              POS starting_position,
                                              int direction_idx, int num_rows,
                                              int num_cols) {
  std::unordered_set<POS> visited;
  POS current_position = starting_position;
  while (on_board(current_position, num_rows, num_cols)) {
    // update visited set with where we are now
    visited.insert(current_position);
    POS new_position = next_position(current_position, direction_idx);
    // if the next position is an obstacle, we need to rotate the direction
    // clockwise once
    if (std::find(obstacles.begin(), obstacles.end(), new_position) !=
        obstacles.end()) {
      direction_idx = (direction_idx + 1) % 4;
    } else {
      // otherwise we move to it
      current_position = new_position;
    }
  }
  return visited;
}

namespace aoc {
void solve_day06_part1(const std::string &input_path) {
  std::vector<std::string> board_content = get_board_rows(input_path);
  std::vector<POS> obstacles = get_obstacles(board_content);
  POS starting_position = get_starting_position(board_content);
  int num_rows, num_cols;
  int direction_idx = 0;
  num_rows = board_content.size();
  num_cols = board_content[0].size();
  if (starting_position == std::pair(-1, -1)) {
    std::cout << "NO STARTING POSITION FOUND" << std::endl;
    return;
  }
  std::unordered_set<POS> visited = get_visited_positions(
      obstacles, starting_position, direction_idx, num_rows, num_cols);
  std::cout << "\nPart 1: " << visited.size() << std::endl;
}
void solve_day06_part2(const std::string &input_path) {
  std::vector<std::string> board_content = get_board_rows(input_path);
  std::vector<POS> obstacles = get_obstacles(board_content);
  POS starting_position = get_starting_position(board_content);
  int num_rows, num_cols;
  num_rows = board_content.size();
  num_cols = board_content[0].size();
  if (starting_position == std::pair(-1, -1)) {
    std::cout << "NO STARTING POSITION FOUND" << std::endl;
    return;
  }
  int direction_idx = 0;
  std::unordered_set<POS> visited = get_visited_positions(
      obstacles, starting_position, direction_idx, num_rows, num_cols);

  int poss_num_loops = 0;
  // for every spot visited, attempt to place an obstacle there
  int total_grid_squares = num_rows * num_cols;
  for (POS pos : visited) {
    obstacles.push_back(pos);
    // this could cause a loop, so using a `while on_board` loop doesn't work.
    // instead we keep track of the number of squares visited in this iteration
    // if this number matches the total area of the board, we made a loop.
    // also current size because with the addition of the new obstacle, the size
    // is n + 1.
    int num_visited = 0;
    // reset to initial state
    POS current_position = starting_position;
    direction_idx = 0;
    // TODO<ryan>: this loop is a little naive, as it requires the path traveled
    // to be longer than the space on the board we can make it faster by
    // comparing the position current to previous positions and their traveling
    // direction. if there is a repeat of (position, direction), then we have a
    // loop
    while (num_visited < total_grid_squares &&
           on_board(current_position, num_rows, num_cols)) {
      num_visited++;
      POS new_position = next_position(current_position, direction_idx);
      if (std::find(obstacles.begin(), obstacles.end(), new_position) !=
          obstacles.end()) {
        // found obstacle
        direction_idx = (direction_idx + 1) % 4;
      } else {
        current_position = new_position;
      }
    }
    if (num_visited >= total_grid_squares) {
      poss_num_loops++;
    }
    obstacles.pop_back();
  }

  std::cout << "\nPart 2: " << poss_num_loops << std::endl;
}
} // namespace aoc
