#include "day18.hpp"
#include "utils/line_reader.hpp"
#include <algorithm>
#include <deque>
#include <iostream>
#include <limits>
#include <ostream>
#include <string>
#include <unordered_set>
#include <vector>

typedef std::pair<int, int> POS;
template <> struct std::hash<std::pair<int, int>> {
  size_t operator()(const std::pair<int, int> &p) const {
    return hash<int>()(p.first) ^ (hash<int>()(p.second) << 1);
  }
};
namespace day18 {
const char *custom_width = std::getenv("GRID_WIDTH");
const char *custom_height = std::getenv("GRID_HEIGHT");
const int grid_width =
    custom_width ? std::stoi(custom_width) : 71; // default is 101
const int grid_height =
    custom_height ? std::stoi(custom_height) : 71; // default is 103
std::vector<POS> neighbors(POS pos) {
  std::vector<POS> neighbors;
  neighbors.push_back(std::make_pair(pos.first - 1, pos.second));
  neighbors.push_back(std::make_pair(pos.first + 1, pos.second));
  neighbors.push_back(std::make_pair(pos.first, pos.second - 1));
  neighbors.push_back(std::make_pair(pos.first, pos.second + 1));
  return neighbors;
}
enum class ByteCell {
  EMPTY,
  CORRUPT,
};
class ByteGrid {
public:
  std::vector<std::vector<ByteCell>> grid;
  POS goal = std::make_pair(grid_height - 1, grid_width - 1);
  ByteGrid() {
    grid.resize(static_cast<size_t>(grid_height),
                std::vector<ByteCell>(static_cast<size_t>(grid_width),
                                      ByteCell::EMPTY));
  }
  void corrupt(POS pos) { grid[pos.first][pos.second] = ByteCell::CORRUPT; }
  bool on_board(POS pos) {
    return pos.first >= 0 && pos.first < grid_height && pos.second >= 0 &&
           pos.second < grid_width;
  }
  bool can_move_to(POS pos) {
    return on_board(pos) && grid[pos.first][pos.second] == ByteCell::EMPTY;
  }
  void print_visited(std::unordered_set<POS> &visited) {
    for (size_t row = 0; row < grid.size(); row++) {
      for (size_t col = 0; col < grid[row].size(); col++) {
        if (visited.contains(std::make_pair(row, col))) {
          std::cout << "O";
        } else if (grid[row][col] == ByteCell::CORRUPT) {
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

      if (curr.first == goal) {
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
};
POS parse_byte(const std::string &s) {
  int row = std::stoi(s.substr(0, s.find(',')));
  int col = std::stoi(s.substr(s.find(',') + 1));
  return std::make_pair(row, col);
}
}; // namespace day18
inline std::ostream &operator<<(std::ostream &os, const day18::ByteGrid &grid) {
  for (size_t row = 0; row < grid.grid.size(); row++) {
    for (size_t col = 0; col < grid.grid[row].size(); col++) {
      switch (grid.grid[row][col]) {
      case day18::ByteCell::EMPTY:
        os << ".";
        break;
      case day18::ByteCell::CORRUPT:
        os << "#";
        break;
      }
    }
    os << "\n";
  }
  return os;
}
namespace aoc {
using namespace day18;

void solve_day18_part1(const std::string &input_path) {
  ByteGrid memory;
  LineIterator it(input_path);
  auto end = LineIterator::end();
  for (int i = 0; i < 1024 && it != end; ++it) {
    memory.corrupt(parse_byte(*it));
    i++;
  }
  auto path = memory.find_path_length(std::make_pair(0, 0));
  std::cout << "\nPart 1: " << path.size() << std::endl;
}
void solve_day18_part2(const std::string &input_path) {
  ByteGrid memory;
  LineIterator it(input_path);
  auto end = LineIterator::end();
  // corrupt original 1024, because we know we can get to the goal
  for (int i = 0; i < 1024 && it != end; ++it) {
    memory.corrupt(parse_byte(*it));
    i++;
  }
  std::string answer = "-1,-1";
  auto path = memory.find_path_length(std::make_pair(0, 0));
  for (; it != end; ++it) {
    POS next_corrupt = parse_byte(*it);
    // if the next corrupt is not in the path, we can corrupt it without
    // re-solving
    memory.corrupt(parse_byte(*it));
    if (std::find(path.begin(), path.end(), next_corrupt) == path.end()) {
      continue;
    };
    // since this byte affected our path we need to resolve
    path = memory.find_path_length(std::make_pair(0, 0));
    // if we can't find a path, that means we have found the byte that ruins it
    if (path.size() == 0) {
      answer = *it;
      break;
    }
  }
  std::cout << "\nPart 2: " << answer << std::endl;
}
}; // namespace aoc
