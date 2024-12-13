#include "day12.hpp"
#include "utils/line_reader.hpp"
#include <functional>
#include <iostream>
#include <stdexcept>
#include <string>
#include <unordered_set>
#include <utility>
#include <vector>

// ROW, COL order
typedef std::pair<int, int> POS;
template <> struct std::hash<POS> {
  size_t operator()(const POS &p) const {
    return hash<int>()(p.first) ^ (hash<int>()(p.second) << 1);
  }
};
enum class Direction : uint8_t { Up, Down, Left, Right };
typedef std::pair<POS, Direction> PerimeterEdge;
Direction indexToDirection(int index) {
  if (index < 0 || index > 3) {
    throw std::out_of_range("Index must be between 0 and 3");
  }
  return static_cast<Direction>(index);
}
template <> struct std::hash<PerimeterEdge> {
  size_t operator()(const PerimeterEdge &pe) const {
    return hash<int>()(static_cast<int>(pe.second)) ^
           (hash<int>()(pe.first.first) << 1) ^
           (hash<int>()(pe.first.second) << 2);
  }
};
namespace day12 {
class GardenRegion {
public:
  char id;
  std::vector<POS> positions;
  long perimeter;
  GardenRegion(char identifier) : id(identifier), positions({}), perimeter(0) {}
  GardenRegion(char identifier, long perimeter)
      : id(identifier), positions({}), perimeter(perimeter) {}
  long get_price() const { return this->get_area() * perimeter; }
  void set_perimeter(long perimeter) { this->perimeter = perimeter; }
  void add_position(const POS &pos) { positions.push_back(pos); }
  long get_area() const { return positions.size(); }
  bool operator==(const GardenRegion &other) const { return id == other.id; }
};

// Add operator<< overload for FloatingStone
inline std::ostream &operator<<(std::ostream &os, const GardenRegion &region) {
  return os << "GardenRegion(id=" << region.id << ", " << region.get_area()
            << ", " << region.perimeter << ")";
}

std::vector<std::vector<char>> read_grid(const std::string &filepath) {
  auto end = LineIterator::end();
  std::vector<std::vector<char>> grid = {{}};
  for (LineIterator it(filepath); it != end; ++it) {
    const auto &line = *it;
    for (char c : line) {
      grid[grid.size() - 1].push_back(c);
    }
    grid.push_back(std::vector<char>());
  }
  if (grid[grid.size() - 1].empty()) {
    grid.pop_back();
  }
  return grid;
}

std::vector<POS> next_cardinal_positions(POS pos) {
  return {std::make_pair(pos.first - 1, pos.second),  // up
          std::make_pair(pos.first + 1, pos.second),  // down
          std::make_pair(pos.first, pos.second - 1),  // left
          std::make_pair(pos.first, pos.second + 1)}; // right
}

bool is_in_bounds(const std::vector<std::vector<char>> &grid, POS pos) {
  int num_rows = grid.size();
  int num_cols = grid[0].size();
  return pos.first >= 0 && pos.first < num_rows && pos.second >= 0 &&
         pos.second < num_cols;
}

GardenRegion get_region_dimensions(const std::vector<std::vector<char>> &grid,
                                   const POS &start_pos,
                                   std::unordered_set<POS> &visited) {
  long perimeter = 0;
  char identifier = grid[start_pos.first][start_pos.second];
  GardenRegion region(identifier);
  // inner lambda, the first pair of square brackets is the capture list
  // makes those variables available inside the lambda
  // could type this as `auto` also
  std::function<void(const POS &pos)> dfs = [&grid, &visited, &perimeter,
                                             &region, &dfs](const POS &pos) {
    if (grid[pos.first][pos.second] != region.id) {
      return;
    }
    if (visited.contains(pos)) {
      return;
    }
    visited.insert(pos);
    region.add_position(pos);

    for (const auto &next_pos : next_cardinal_positions(pos)) {
      if (is_in_bounds(grid, next_pos) &&
          grid[next_pos.first][next_pos.second] == region.id) {
        dfs(next_pos);
      } else {
        perimeter++;
      }
    }
  };
  dfs(start_pos);
  region.set_perimeter(perimeter);
  return region;
}
std::vector<GardenRegion>
build_regions(const std::vector<std::vector<char>> &grid) {
  std::unordered_set<POS> visited;
  std::vector<GardenRegion> regions;
  for (size_t row = 0; row < grid.size(); row++) {
    for (size_t col = 0; col < grid[row].size(); col++) {
      if (!visited.contains(std::make_pair(row, col))) {
        GardenRegion region =
            get_region_dimensions(grid, std::make_pair(row, col), visited);
        regions.push_back(region);
      }
    }
  }
  return regions;
}

long calculate_sides(const GardenRegion &region) {
  std::unordered_set<PerimeterEdge> edges;
  for (const auto &pos : region.positions) {
    std::vector<POS> cardinal_positions = next_cardinal_positions(pos);
    for (size_t i = 0; i < cardinal_positions.size(); i++) {
      POS next_pos = cardinal_positions[i];
      Direction direction = indexToDirection(i);
      if (std::find(region.positions.begin(), region.positions.end(),
                    next_pos) == region.positions.end()) {
        edges.insert(std::make_pair(pos, direction));
      }
    }
  }
  long sides = 0;
  for (const PerimeterEdge &edge : edges) {
    switch (edge.second) {
    case Direction::Up:
      // check if there point to right is part of same side
      if (!edges.contains(std::make_pair(
              std::make_pair(edge.first.first, edge.first.second + 1),
              Direction::Up))) {
        sides += 1;
      }
      break;
    case Direction::Down:
      // check if there point to right is part of same side
      if (!edges.contains(std::make_pair(
              std::make_pair(edge.first.first, edge.first.second + 1),
              Direction::Down))) {
        sides += 1;
      }
      break;
    case Direction::Left:
      // check if there point below is part of same side
      if (!edges.contains(std::make_pair(
              std::make_pair(edge.first.first + 1, edge.first.second),
              Direction::Left))) {
        sides += 1;
      }
      break;
    case Direction::Right:
      // check if there point below is part of same side
      if (!edges.contains(std::make_pair(
              std::make_pair(edge.first.first + 1, edge.first.second),
              Direction::Right))) {
        sides += 1;
      }
      break;
    }
  }
  return sides;
}
}; // namespace day12
namespace aoc {
using namespace day12;

void solve_day12_part1(const std::string &input_path) {
  auto grid = read_grid(input_path);
  std::vector<GardenRegion> regions = build_regions(grid);
  long total_price = 0;
  for (const GardenRegion &region : regions) {
    total_price += region.get_price();
  }
  std::cout << "\nPart 1: " << total_price << std::endl;
}
void solve_day12_part2(const std::string &input_path) {
  auto grid = read_grid(input_path);
  std::vector<GardenRegion> regions = build_regions(grid);
  long total_price = 0;
  for (const GardenRegion &region : regions) {
    total_price += (calculate_sides(region) * region.get_area());
  }
  std::cout << "\nPart 2: " << total_price << std::endl;
}
}; // namespace aoc
