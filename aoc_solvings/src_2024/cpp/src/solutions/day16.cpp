#include "day16.hpp"
#include "utils/input_reader.hpp"
#include <functional>
#include <iomanip>
#include <iostream>
#include <limits>
#include <optional>
#include <queue>
#include <string>
#include <unordered_set>
#include <utility>
#include <variant>
#include <vector>

template <> struct std::hash<std::pair<int, int>> {
  size_t operator()(const std::pair<int, int> &p) const {
    return hash<int>()(p.first) ^ (hash<int>()(p.second) << 1);
  }
};
namespace day16 {
enum class Direction {
  UP = 0,
  RIGHT = 1,
  DOWN = 2,
  LEFT = 3,
};
inline std::ostream &operator<<(std::ostream &os, const Direction &dir) {
  switch (dir) {
  case Direction::UP:
    return os << "^";
  case Direction::DOWN:
    return os << "v";
  case Direction::LEFT:
    return os << "<";
  case Direction::RIGHT:
    return os << ">";
  }
};
int operator-(Direction from, Direction to) {
  return static_cast<int>(from) - static_cast<int>(to);
}
int turn_cost(Direction from, Direction to) { return from == to ? 0 : 1000; }
enum class Cell {
  EMPTY,
  WALL,
  END,
};
void print_dist(const std::vector<std::vector<long>> &dist) {
  for (const std::vector<long> &row : dist) {
    for (long val : row) {
      std::cout << std::setw(7) << val << " ";
    }
    std::cout << "\n";
  }
};
void print_dir_dist(const std::vector<std::vector<std::vector<long>>> &dist) {
  for (const auto &row : dist) {
    for (auto val : row) {
      long min = std::min(val[0], std::min(val[1], std::min(val[2], val[3])));
      long output = min == std::numeric_limits<long>::max() ? -1 : min;
      std::cout << std::setw(7) << output << " ";
    }
    std::cout << "\n";
  }
};
struct DjikstraBNode {
  std::pair<int, int> position;
  Direction direction;
  long cost;
  std::vector<std::pair<int, int>> path;
  auto operator<=>(const DjikstraBNode &other) const {
    return cost <=> other.cost;
  };
};
class Maze {
public:
  std::pair<int, int> start, end;
  std::vector<std::vector<Cell>> cells;
  Maze(const std::string &content) {
    cells = {{}};
    for (char c : content) {
      switch (c) {
      case '\n':
        cells.push_back(std::vector<Cell>());
        break;
      case '.':
        cells[cells.size() - 1].push_back(Cell::EMPTY);
        break;
      case '#':
        cells[cells.size() - 1].push_back(Cell::WALL);
        break;
      case 'S':
        start =
            std::make_pair(cells.size() - 1, cells[cells.size() - 1].size());
        cells[cells.size() - 1].push_back(Cell::EMPTY);
        break;
      case 'E':
        end = std::make_pair(cells.size() - 1, cells[cells.size() - 1].size());
        cells[cells.size() - 1].push_back(Cell::END);
        break;
      }
    }
    // check that last line is not empty
    if (cells[cells.size() - 1].empty()) {
      cells.pop_back();
    }
  }
  std::vector<std::pair<std::pair<int, int>, Direction>>
  get_neighbors(std::pair<int, int> pos) {
    return {
        {std::make_pair(pos.first - 1, pos.second), Direction::UP},
        {std::make_pair(pos.first, pos.second + 1), Direction::RIGHT},
        {std::make_pair(pos.first + 1, pos.second), Direction::DOWN},
        {std::make_pair(pos.first, pos.second - 1), Direction::LEFT},
    };
  }
  bool can_move_to(std::pair<int, int> pos) {
    // bounds check and make sure is not a wall
    return pos.first >= 0 && pos.first < static_cast<int>(cells.size()) &&
           pos.second >= 0 && pos.second < static_cast<int>(cells[0].size()) &&
           cells[pos.first][pos.second] != Cell::WALL;
  }
  long djikstra() {
    std::vector<std::vector<long>> dist(cells.size(),
                                        std::vector<long>(cells[0].size(), -1));
    // has to be min heap impl, SCORE, ROW, COL, DIRECTIOn
    std::priority_queue<std::tuple<int, int, int, Direction>,
                        std::vector<std::tuple<int, int, int, Direction>>,
                        std::greater<std::tuple<int, int, int, Direction>>>
        q;
    q.push({0, start.first, start.second, Direction::RIGHT});
    dist[start.first][start.second] = 0;
    while (!q.empty()) {
      std::tuple<int, int, int, Direction> curr = q.top();
      std::pair<int, int> curr_position =
          std::make_pair(std::get<1>(curr), std::get<2>(curr));
      Direction curr_direction = std::get<3>(curr);
      q.pop();
      if (curr_position == end) {
        return dist[curr_position.first][curr_position.second];
      }
      for (auto [neighbor, dir] : get_neighbors(curr_position)) {
        if (can_move_to(neighbor)) {
          long new_dist = dist[curr_position.first][curr_position.second] + 1 +
                          turn_cost(curr_direction, dir);
          if (dist[neighbor.first][neighbor.second] == -1 ||
              new_dist < dist[neighbor.first][neighbor.second]) {
            dist[neighbor.first][neighbor.second] = new_dist;
            q.push({new_dist, neighbor.first, neighbor.second, dir});
          }
        }
      }
    }
    return -1;
  }
  std::unordered_set<std::pair<int, int>> djikstra_path() {
    std::vector<std::vector<std::vector<long>>> dist(
        cells.size(), std::vector<std::vector<long>>(
                          cells[0].size(), {std::numeric_limits<long>::max(),
                                            std::numeric_limits<long>::max(),
                                            std::numeric_limits<long>::max(),
                                            std::numeric_limits<long>::max()}));
    std::unordered_set<std::pair<int, int>> visited;
    std::optional<long> best_path_cost;
    // has to be min heap impl, SCORE, ROW, COL, DIRECTIOn
    std::priority_queue<DjikstraBNode, std::vector<DjikstraBNode>,
                        std::greater<>>
        q;
    q.push({start, Direction::RIGHT, 0, {start}});
    dist[start.first][start.second][static_cast<int>(Direction::RIGHT)] = 0;
    while (!q.empty()) {
      DjikstraBNode curr = q.top();
      q.pop();
      curr.path.push_back(curr.position);
      // in this part we need to find all possible paths
      if (curr.position == end) {
        if (best_path_cost.has_value() && best_path_cost.value() < curr.cost) {
          // found path that is not the best so we have already found all the
          // possible best paths
          break;
        }
        for (auto pos : curr.path) {
          visited.insert(pos);
        }
        // possible that best_path_cost is not Some yet, so set here
        best_path_cost = curr.cost;
        continue;
      }
      for (auto [neighbor, dir] : get_neighbors(curr.position)) {
        if (can_move_to(neighbor)) {
          long new_dist = curr.cost + 1 + turn_cost(curr.direction, dir);
          // note usage of lte over lt, need to revisit nodes if equivalent
          // path is found
          if (new_dist <=
              dist[neighbor.first][neighbor.second][static_cast<int>(dir)]) {
            dist[neighbor.first][neighbor.second][static_cast<int>(dir)] =
                new_dist;
            q.push({
                neighbor,
                dir,
                new_dist,
                curr.path,
            });
          }
        }
      }
    }
    return visited;
  }
  void display_visited(const std::unordered_set<std::pair<int, int>> &visited) {
    for (size_t i = 0; i < cells.size(); i++) {
      for (size_t j = 0; j < cells[0].size(); j++) {
        if (visited.find(std::make_pair(i, j)) != visited.end()) {
          std::cout << "O";
        } else if (cells[i][j] == Cell::WALL) {
          std::cout << "#";
        } else {
          std::cout << ".";
        }
      }
      std::cout << "\n";
    }
  }
};
}; // namespace day16
namespace aoc {
using namespace day16;

void solve_day16_part1(const std::string &input_path) {
  std::string content = FileReader::readFile(input_path);
  Maze maze(content);
  long distance = maze.djikstra();
  if (distance == -1) {
    std::cout << "NO PATH FOUND" << std::endl;
    return;
  }
  std::cout << "\nPart 1: " << distance << std::endl;
}
void solve_day16_part2(const std::string &input_path) {
  std::string content = FileReader::readFile(input_path);
  Maze maze(content);
  std::unordered_set<std::pair<int, int>> visited = maze.djikstra_path();
  std::cout << "\nPart 2: " << visited.size() << std::endl;
}
}; // namespace aoc
