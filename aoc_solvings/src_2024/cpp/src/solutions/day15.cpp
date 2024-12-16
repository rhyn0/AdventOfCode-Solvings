#include "day15.hpp"
#include "utils/input_reader.hpp"
#include <cstddef>
#include <cstring>
#include <deque>
#include <iostream>
#include <string>
#include <unordered_set>
#include <utility>
#include <vector>

template <> struct std::hash<std::pair<int, int>> {
  size_t operator()(const std::pair<int, int> &p) const {
    return hash<int>()(p.first) ^ (hash<int>()(p.second) << 1);
  }
};
namespace day15 {
enum class Direction {
  UP,
  DOWN,
  LEFT,
  RIGHT,
};
enum class Cell {
  EMPTY,
  WALL,
  BOX,
  ROBOT,
  LEFT_BOX,
  RIGHT_BOX,
};

std::pair<int, int> next_position(const std::pair<int, int> position,
                                  Direction direction) {
  switch (direction) {
  case Direction::UP:
    return std::make_pair(position.first - 1, position.second);
  case Direction::DOWN:
    return std::make_pair(position.first + 1, position.second);
  case Direction::LEFT:
    return std::make_pair(position.first, position.second - 1);
  case Direction::RIGHT:
    return std::make_pair(position.first, position.second + 1);
  }
  return std::make_pair(-1, -1);
}
class Map {
public:
  std::vector<std::vector<Cell>> grid;
  Map() {};
  Map(const std::string &content, bool part2) {
    grid = {{}};
    // part2 enables almost input characters to be counted as 2 cells
    // walls -> 2 walls next to each other
    // boxes -> left side box and right side box
    // robot -> empty to the right
    // empty -> 2 empty
    for (char c : content) {
      switch (c) {
      case '#':
        grid[grid.size() - 1].push_back(Cell::WALL);
        if (part2) {
          grid[grid.size() - 1].push_back(Cell::WALL);
        }
        break;
      case '.':
        grid[grid.size() - 1].push_back(Cell::EMPTY);
        if (part2) {
          grid[grid.size() - 1].push_back(Cell::EMPTY);
        }
        break;
      case '@':
        grid[grid.size() - 1].push_back(Cell::ROBOT);
        if (part2) {
          grid[grid.size() - 1].push_back(Cell::EMPTY);
        }
        break;
      case 'O':
        if (part2) {
          grid[grid.size() - 1].push_back(Cell::LEFT_BOX);
          grid[grid.size() - 1].push_back(Cell::RIGHT_BOX);
        } else {
          grid[grid.size() - 1].push_back(Cell::BOX);
        }
        break;
      case '\n':
        grid.push_back(std::vector<Cell>());
        break;
      default:
        break;
      }
    }
    if (grid[grid.size() - 1].empty()) {
      grid.pop_back();
    }
  }
  std::pair<int, int> get_robot_position() {
    for (size_t row = 0; row < grid.size(); row++) {
      for (size_t col = 0; col < grid[row].size(); col++) {
        if (grid[row][col] == Cell::ROBOT) {
          return std::make_pair(row, col);
        }
      }
    }
    return std::make_pair(-1, -1);
  }
  Cell get_cell(std::pair<int, int> position) {
    return grid[position.first][position.second];
  }
  bool on_board(std::pair<int, int> position) {
    return position.first >= 0 &&
           position.first < static_cast<int>(grid.size()) &&
           position.second >= 0 &&
           position.second < static_cast<int>(grid[0].size());
  }
  std::pair<int, int> check_direction(std::pair<int, int> position,
                                      Direction direction) {
    std::pair<int, int> curr_position = position;
    // while in a line of boxes, keep pushing them all in the desired direction
    while (on_board(curr_position) && get_cell(curr_position) == Cell::BOX) {
      if (direction == Direction::UP) {
        curr_position.first -= 1;
      }
      if (direction == Direction::DOWN) {
        curr_position.first += 1;
      }
      if (direction == Direction::LEFT) {
        curr_position.second -= 1;
      }
      if (direction == Direction::RIGHT) {
        curr_position.second += 1;
      }
    }
    // either we hit a wall or the `curr_position` is an empty cell
    // still be safe by checking for out of bounds though it should be
    // impossible when grid is surrounded by walls
    if (!on_board(curr_position) || get_cell(curr_position) == Cell::WALL) {
      return std::make_pair(-1, -1);
    }
    return curr_position;
  }
  /**
   * @brief Return whether the move was successful
   *
   * @param proposed_position - position to move to
   * @param direction - direction that moves to `proposed_position`
   * @return true if move was successful
   * @return false if move was not successful
   */
  bool make_move(std::pair<int, int> proposed_position, Direction direction) {
    Cell move_to_cell = get_cell(proposed_position);
    if (move_to_cell == Cell::WALL) {
      return false;
    }
    if (move_to_cell == Cell::EMPTY) {
      return true;
    }
    // new position is a box
    // boxes can be moved if there is an EMPTY in the DIRECTION given
    std::pair<int, int> empty_position =
        check_direction(proposed_position, direction);
    if (empty_position == std::make_pair(-1, -1)) {
      return false;
    }
    grid[empty_position.first][empty_position.second] = Cell::BOX;
    grid[proposed_position.first][proposed_position.second] = Cell::EMPTY;
    return true;
  }
  /**
   * @brief Return whether the move was successful
   *
   * Introduces logic for the extra big boxes to be moved. Necessary since part2
   * boxes are 2 Cells.
   *
   * @param proposed_position - position to move to
   * @param direction - direction that moves to `proposed_position`
   */
  void make_move_part2(Direction direction) {
    // same base cases as `make_move`
    std::pair<int, int> robot_position = get_robot_position();
    Cell move_to_cell = get_cell(robot_position);
    if (move_to_cell == Cell::WALL) {
      return;
    }
    if (move_to_cell == Cell::EMPTY) {
      return;
    }
    // we have to find all cells that would be interacted with
    // for the proposed move
    std::deque<std::pair<int, int>> cells_to_move;
    std::unordered_set<std::pair<int, int>> visited;
    cells_to_move.push_back(robot_position);
    while (!cells_to_move.empty()) {
      std::pair<int, int> curr_position = cells_to_move.front();
      cells_to_move.pop_front();
      if (!visited.insert(curr_position).second) {
        continue;
      }
      std::pair<int, int> next_pos = next_position(curr_position, direction);
      Cell curr_cell = get_cell(next_pos);
      switch (curr_cell) {
      // Left box is tied to a Right box, so both must move
      case Cell::LEFT_BOX:
        // use `next_position` to add the spot at which
        // this Cell would be moved to
        cells_to_move.push_back(next_pos);
        // add the right box to the queue
        cells_to_move.push_back(
            std::make_pair(next_pos.first, next_pos.second + 1));
        break;
      case Cell::RIGHT_BOX:
        // use `next_position` to add the spot at which
        // this Cell would be moved to
        cells_to_move.push_back(next_pos);
        // add the left box to the queue
        cells_to_move.push_back(
            std::make_pair(next_pos.first, next_pos.second - 1));
        break;
      case Cell::EMPTY:
        continue;
      case Cell::WALL:
        // any collision into a wall is a failure to move, can't do anything
        return;
      case Cell::ROBOT:
      case Cell::BOX:
        throw std::runtime_error("INVALID BOX AT " +
                                 std::to_string(curr_position.first) + " " +
                                 std::to_string(curr_position.second));
      default:
        break;
      }
    }
    // brute force move all cells that were visited
    // forcibly iterates over all entries to find the set of cells
    // that don't modify to be moved cells.
    // Essentially if A moves B which moves C.
    // we have a graph of A -> B -> C
    // so we need to move C first then B then A
    while (!visited.empty()) {
      for (auto &pos : visited) {
        std::pair<int, int> next_pos = next_position(pos, direction);
        if (!visited.contains(next_pos)) {
          grid[next_pos.first][next_pos.second] = grid[pos.first][pos.second];
          grid[pos.first][pos.second] = Cell::EMPTY;
          visited.erase(pos);
        }
      }
    }
  }
  long get_gps_score(bool part2 = false) {
    long score = 0;
    Cell desired_cell = part2 ? Cell::LEFT_BOX : Cell::BOX;
    for (size_t row = 0; row < grid.size(); row++) {
      for (size_t col = 0; col < grid[row].size(); col++) {
        if (grid[row][col] == desired_cell) {
          score += (row * 100) + col;
        }
      }
    }
    return score;
  }
  void set_robot_position(std::pair<int, int> position) {
    std::pair<int, int> robot_position = get_robot_position();
    grid[robot_position.first][robot_position.second] = Cell::EMPTY;
    grid[position.first][position.second] = Cell::ROBOT;
  }
};
inline std::ostream &operator<<(std::ostream &os, const Map &map) {
  std::string output = "";
  for (size_t row = 0; row < map.grid.size(); row++) {
    for (size_t col = 0; col < map.grid[row].size(); col++) {
      switch (map.grid[row][col]) {
      case Cell::EMPTY:
        output += ".";
        break;
      case Cell::WALL:
        output += "#";
        break;
      case Cell::ROBOT:
        output += "@";
        break;
      case Cell::BOX:
        output += "O";
        break;
      case Cell::LEFT_BOX:
        output += "[";
        break;
      case Cell::RIGHT_BOX:
        output += "]";
        break;
      }
    }
    output += "\n";
  }
  return os << output;
};
std::vector<Direction> parse_directions(const std::string &content) {
  std::vector<Direction> directions;
  for (char c : content) {
    switch (c) {
    case '\n':
      break;
    case '>':
      directions.push_back(Direction::RIGHT);
      break;
    case '<':
      directions.push_back(Direction::LEFT);
      break;
    case '^':
      directions.push_back(Direction::UP);
      break;
    case 'v':
      directions.push_back(Direction::DOWN);
      break;
    default:
      break;
    }
  }
  return directions;
}
std::pair<Map, std::vector<Direction>> parse_input(const std::string &filepath,
                                                   bool part2 = false) {
  std::string content = FileReader::readFile(filepath);
  // two sections, split by one empty line (two newlines in a row)
  size_t split_index = content.find("\n\n");
  if (split_index == std::string::npos) {
    throw std::runtime_error("invalid input");
  }
  std::string map = content.substr(0, split_index);
  std::string directions_string = content.substr(split_index + 2);
  Map grid(map, part2);
  std::vector<Direction> directions = parse_directions(directions_string);
  return std::make_pair(grid, directions);
}

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
}; // namespace day15
namespace aoc {
using namespace day15;

void solve_day15_part1(const std::string &input_path) {
  Map grid;
  std::vector<Direction> directions;
  std::tie(grid, directions) = parse_input(input_path);
  std::pair<int, int> robot_position = grid.get_robot_position();
  std::pair<int, int> next_pos;
  for (const Direction &direction : directions) {
    next_pos = next_position(robot_position, direction);
    if (grid.make_move(next_pos, direction)) {
      grid.set_robot_position(next_pos);
      robot_position = next_pos;
    }
  }
  std::cout << "\nPart 1: " << grid.get_gps_score() << std::endl;
}
void solve_day15_part2(const std::string &input_path) {
  Map grid;
  std::vector<Direction> directions;
  std::tie(grid, directions) = parse_input(input_path, true);
  for (const Direction &direction : directions) {
    grid.make_move_part2(direction);
  }
  std::cout << "\nPart 2: " << grid.get_gps_score(true) << std::endl;
}
}; // namespace aoc
