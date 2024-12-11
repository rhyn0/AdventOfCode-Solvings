#include "day10.hpp"
#include "utils/input_reader.hpp"
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

// ROW, COL order
typedef std::pair<int, int> POS;
namespace std {
template <> struct hash<POS> {
  size_t operator()(const POS &p) const {
    return hash<int>()(p.first) ^ (hash<int>()(p.second) << 1);
  }
};
}; // namespace std
namespace day10 {
class TrailGrid {
public:
  TrailGrid(const std::string &filepath) {
    std::string content = FileReader::readFile(filepath);
    grid = {{}};
    for (char c : content) {
      if (c == '\n') {
        grid.push_back(std::vector<short>());
      } else {
        grid[grid.size() - 1].push_back(c - '0');
      }
    }
    if (grid[grid.size() - 1].empty()) {
      grid.pop_back();
    }
  }
  short get(POS pos) { return grid[pos.first][pos.second]; }
  size_t get_num_rows() { return grid.size(); }
  size_t get_num_cols() { return grid[0].size(); }
  bool is_in_bounds(POS pos) {
    return pos.first >= 0 && pos.first < static_cast<int>(grid.size()) &&
           pos.second >= 0 && pos.second < static_cast<int>(grid[0].size());
  }

private:
  std::vector<std::vector<short>> grid;
};

std::vector<POS> next_cardinal_positions(POS pos) {
  return {std::make_pair(pos.first - 1, pos.second),
          std::make_pair(pos.first + 1, pos.second),
          std::make_pair(pos.first, pos.second - 1),
          std::make_pair(pos.first, pos.second + 1)};
}
void dfs_trails(TrailGrid &grid, std::unordered_set<POS> &peaks, POS pos) {
  if (!grid.is_in_bounds(pos)) {
    return;
  }
  if (grid.get(pos) == 9) {
    peaks.insert(pos);
    return;
  }
  short current_height = grid.get(pos);
  for (POS next_pos : next_cardinal_positions(pos)) {
    if (grid.is_in_bounds(next_pos) &&
        grid.get(next_pos) == current_height + 1) {
      dfs_trails(grid, peaks, next_pos);
    }
  }
}
/**
 * Part 2 - return the rating of a trailhead. Number of distinct paths to get to
 * a peak.
 */
int dfs_trails_part_2(TrailGrid &grid, std::unordered_set<POS> &visited,
                      POS pos) {
  if (!grid.is_in_bounds(pos) || visited.find(pos) != visited.end()) {
    return 0;
  }
  if (grid.get(pos) == 9) {
    return 1;
  }
  visited.insert(pos);
  short current_height = grid.get(pos);
  int trailhead_rating = 0;
  for (POS next_pos : next_cardinal_positions(pos)) {
    if (grid.is_in_bounds(next_pos) &&
        grid.get(next_pos) == current_height + 1) {
      trailhead_rating += dfs_trails_part_2(grid, visited, next_pos);
    }
  }
  visited.erase(pos);
  return trailhead_rating;
}
}; // namespace day10
namespace aoc {
using namespace day10;

void solve_day10_part1(const std::string &input_path) {
  TrailGrid grid(input_path);
  int running_trailhead_score = 0;
  for (size_t row = 0; row < grid.get_num_rows(); row++) {
    for (size_t col = 0; col < grid.get_num_cols(); col++) {
      // only search from valid trailhead heights
      short entry_height = grid.get(std::make_pair(row, col));
      if (entry_height == 0) {
        std::unordered_set<POS> peaks;
        dfs_trails(grid, peaks, std::make_pair(row, col));
        running_trailhead_score += peaks.size();
      }
    }
  }
  std::cout << "\nPart 1: " << running_trailhead_score << std::endl;
}
void solve_day10_part2(const std::string &input_path) {
  TrailGrid grid(input_path);
  int running_trailhead_rating = 0;
  for (size_t row = 0; row < grid.get_num_rows(); row++) {
    for (size_t col = 0; col < grid.get_num_cols(); col++) {
      // only search from valid trailhead heights
      short entry_height = grid.get(std::make_pair(row, col));
      if (entry_height == 0) {
        std::unordered_set<POS> visited;
        int curr_rating =
            dfs_trails_part_2(grid, visited, std::make_pair(row, col));
        running_trailhead_rating += curr_rating;
      }
    }
  }
  std::cout << "\nPart 2: " << running_trailhead_rating << std::endl;
}
}; // namespace aoc
