#include "day20.hpp"
#include "utils/input_reader.hpp"
#include <algorithm>
#include <deque>
#include <iostream>
#include <string>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#define CHEAT_THRESHOLD 100

typedef std::pair<int, int> POS;
template <> struct std::hash<std::pair<int, int>> {
  size_t operator()(const std::pair<int, int> &p) const {
    return hash<int>()(p.first) ^ (hash<int>()(p.second) << 1);
  }
};
template <> struct std::hash<std::pair<POS, POS>> {
  size_t operator()(const std::pair<POS, POS> &p) const {
    return hash<POS>()(p.first) ^ (hash<POS>()(p.second) << 1);
  }
};
namespace day20 {
std::vector<POS> neighbors(POS pos) {
  std::vector<POS> neighbors;
  neighbors.push_back(std::make_pair(pos.first - 1, pos.second));
  neighbors.push_back(std::make_pair(pos.first + 1, pos.second));
  neighbors.push_back(std::make_pair(pos.first, pos.second - 1));
  neighbors.push_back(std::make_pair(pos.first, pos.second + 1));
  return neighbors;
}
enum class RaceCell {
  EMPTY,
  WALL,
};
int manhattan_distance(POS p1, POS p2) {
  return abs(p1.first - p2.first) + abs(p1.second - p2.second);
}
class RaceGrid {
public:
  std::vector<std::vector<RaceCell>> grid;
  std::vector<std::vector<int>> distances_from_start;
  POS start, end;
  RaceGrid(const std::string &filepath) {
    std::string content = FileReader::readFile(filepath);
    grid = {{}};
    for (char c : content) {
      if (c == '\n') {
        grid.push_back(std::vector<RaceCell>());
      } else {
        if (c == 'S') {
          start = std::make_pair(grid.size() - 1, grid[grid.size() - 1].size());
        } else if (c == 'E') {
          end = std::make_pair(grid.size() - 1, grid[grid.size() - 1].size());
        }
        grid[grid.size() - 1].push_back(c == '#' ? RaceCell::WALL
                                                 : RaceCell::EMPTY);
      }
    }
    if (grid[grid.size() - 1].empty()) {
      grid.pop_back();
    }
    distances_from_start = std::vector<std::vector<int>>(
        grid.size(),
        std::vector<int>(grid[0].size(), std::numeric_limits<int>::max()));
  }
  void set_distances_from_start(std::vector<POS> path) {
    distances_from_start[start.first][start.second] = 0;
    for (int i = 0; i < static_cast<int>(path.size()); i++) {
      POS pos = path[i];
      distances_from_start[pos.first][pos.second] = i + 1;
    }
  }
  bool on_board(POS pos) {
    return pos.first >= 0 && pos.first < static_cast<int>(grid.size()) &&
           pos.second >= 0 && pos.second < static_cast<int>(grid[0].size());
  }
  bool can_move_to(POS pos) {
    return on_board(pos) && grid[pos.first][pos.second] == RaceCell::EMPTY;
  }
  void print_visited(std::unordered_set<POS> &visited) {
    for (size_t row = 0; row < grid.size(); row++) {
      for (size_t col = 0; col < grid[row].size(); col++) {
        if (visited.contains(std::make_pair(row, col))) {
          std::cout << "O";
        } else if (grid[row][col] == RaceCell::WALL) {
          std::cout << "#";
        } else {
          std::cout << ".";
        }
      }
      std::cout << "\n";
    }
  }
  std::vector<POS> find_path_length(POS curr_pos) {
    std::deque<std::pair<POS, std::vector<POS>>> queue = {
        std::make_pair(curr_pos, std::vector<POS>())};
    std::unordered_set<POS> visited;
    visited.insert(curr_pos); // Mark starting position as visited

    while (!queue.empty()) {
      std::pair<POS, std::vector<POS>> curr = queue.front();
      queue.pop_front();

      if (curr.first == this->end) {
        return curr.second;
      }

      for (POS neighbor : neighbors(curr.first)) {
        if (can_move_to(neighbor) && visited.find(neighbor) == visited.end()) {
          visited.insert(neighbor); // Mark as visited when adding to queue
          // copy the vector and add the neighbor
          std::vector<POS> new_path = curr.second;
          new_path.push_back(neighbor);
          queue.push_back(std::make_pair(neighbor, new_path));
        }
      }
    }
    return std::vector<POS>();
  }
  RaceCell get_cell(POS pos) { return grid[pos.first][pos.second]; }
  std::vector<POS> find_cheat_from_point(POS point, int max_cheat_length = 2) {
    std::vector<POS> ret;
    for (int row = point.first - max_cheat_length;
         row <= point.first + max_cheat_length; row++) {
      for (int col = point.second - max_cheat_length;
           col <= point.second + max_cheat_length; col++) {
        if (row == point.first && col == point.second) {
          continue;
        }
        if (!on_board({row, col}) || grid[row][col] == RaceCell::WALL ||
            manhattan_distance(point, {row, col}) > max_cheat_length) {
          continue;
        }
        ret.push_back(std::make_pair(row, col));
      }
    }
    return ret;
  }
  int calculate_cheat_distance(POS start, POS end) {
    int start_dist = distances_from_start[start.first][start.second];
    int end_dist = distances_from_start[end.first][end.second];
    if (start_dist == std::numeric_limits<int>::max() ||
        end_dist == std::numeric_limits<int>::max()) {
      return -1;
    }
    return end_dist - start_dist - manhattan_distance(start, end);
  }
};
}; // namespace day20
inline std::ostream &operator<<(std::ostream &os, const day20::RaceGrid &grid) {
  for (size_t row = 0; row < grid.grid.size(); row++) {
    for (size_t col = 0; col < grid.grid[row].size(); col++) {
      switch (grid.grid[row][col]) {
      case day20::RaceCell::EMPTY:
        os << ".";
        break;
      case day20::RaceCell::WALL:
        os << "#";
        break;
      }
    }
    os << "\n";
  }
  return os;
}
namespace aoc {
using namespace day20;

void solve_day20_part1(const std::string &input_path) {
  RaceGrid grid(input_path);
  std::vector<POS> path = grid.find_path_length(grid.start);
  grid.set_distances_from_start(path);
  std::unordered_map<std::pair<POS, POS>, int> cheat_map;
  int above_threshold_cheats = 0;
  // path doesn't include the start point, which is a valid cheating spot
  for (auto cheat_end : grid.find_cheat_from_point(grid.start)) {
    auto map_key = std::make_pair(grid.start, cheat_end);
    // note the negative one here, because the start is BEFORE the taken path
    int cheat_distance_savings =
        grid.calculate_cheat_distance(grid.start, cheat_end);
    if (cheat_distance_savings <= 0) {
      // no savings in this case
      continue;
    }
    if (!cheat_map.contains(map_key)) {
      // if the savings are better than the previous one, replace it
      cheat_map[map_key] = cheat_distance_savings;
    } else if (cheat_map[map_key] < cheat_distance_savings) {
      cheat_map[map_key] = cheat_distance_savings;
    }
  }
  for (POS point : path) {
    for (auto cheat_end : grid.find_cheat_from_point(point)) {
      int cheat_distance_savings =
          grid.calculate_cheat_distance(point, cheat_end);
      auto map_key = std::make_pair(point, cheat_end);
      if (cheat_distance_savings <= 0) {
        // no savings in this case
        continue;
      }
      if (cheat_map.find(map_key) == cheat_map.end()) {
        // insert because it doesn't exist
        cheat_map[map_key] = cheat_distance_savings;
      } else {
        // if the savings are better than the previous one, replace it
        if (cheat_map[map_key] < cheat_distance_savings) {
          cheat_map[map_key] = cheat_distance_savings;
        }
      }
    }
  }
  for (auto [key, value] : cheat_map) {
    if (value >= CHEAT_THRESHOLD) {
      above_threshold_cheats++;
    }
  }
  std::cout << "\nPart 1: " << above_threshold_cheats << std::endl;
}
void solve_day20_part2(const std::string &input_path) {
  RaceGrid grid(input_path);
  std::vector<POS> path = grid.find_path_length(grid.start);
  grid.set_distances_from_start(path);
  std::unordered_map<std::pair<POS, POS>, int> cheat_map;
  int above_threshold_cheats = 0;
  // path doesn't include the start point, which is a valid cheating spot
  for (auto cheat_end : grid.find_cheat_from_point(grid.start, 20)) {
    auto map_key = std::make_pair(grid.start, cheat_end);
    // note the negative one here, because the start is BEFORE the taken path
    int cheat_distance_savings =
        grid.calculate_cheat_distance(grid.start, cheat_end);
    if (cheat_distance_savings <= 0) {
      // no savings in this case
      continue;
    }
    if (!cheat_map.contains(map_key)) {
      // if the savings are better than the previous one, replace it
      cheat_map[map_key] = cheat_distance_savings;
    } else if (cheat_map[map_key] < cheat_distance_savings) {
      cheat_map[map_key] = cheat_distance_savings;
    }
  }
  for (int i = 0; i < static_cast<int>(path.size()); i++) {
    POS point = path[i];
    for (auto cheat_end : grid.find_cheat_from_point(point, 20)) {
      int cheat_distance_savings =
          grid.calculate_cheat_distance(point, cheat_end);
      auto map_key = std::make_pair(point, cheat_end);
      if (cheat_distance_savings <= 0) {
        // no savings in this case
        continue;
      }
      if (cheat_map.find(map_key) == cheat_map.end()) {
        // insert because it doesn't exist
        cheat_map[map_key] = cheat_distance_savings;
      } else {
        // if the savings are better than the previous one, replace it
        if (cheat_map[map_key] < cheat_distance_savings) {
          cheat_map[map_key] = cheat_distance_savings;
        }
      }
    }
  }
  for (auto [key, value] : cheat_map) {
    if (value >= CHEAT_THRESHOLD) {
      above_threshold_cheats++;
    }
  }
  std::cout << "\nPart 2: " << above_threshold_cheats << std::endl;
}
}; // namespace aoc
