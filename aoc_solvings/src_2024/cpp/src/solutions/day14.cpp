#include "day14.hpp"
#include "utils/line_reader.hpp"
#include <algorithm>
#include <cmath>
#include <iostream>
#include <limits>
#include <regex>
#include <string>
#include <unordered_map>
#include <vector>

namespace day14 {
// Grid size configuration from environment variables
const char *custom_width = std::getenv("GRID_WIDTH");
const char *custom_height = std::getenv("GRID_HEIGHT");
const int grid_width =
    custom_width ? std::stoi(custom_width) : 101; // default is 101
const int grid_height =
    custom_height ? std::stoi(custom_height) : 103; // default is 103

struct Position {
  int x, y;
};
struct Robot {
  Position pos;
  int vel_x, vel_y;
};
class Day14Grid {
public:
  int width, height;
  Day14Grid(int width, int height) : width(width), height(height) {}
  /**
   * @brief Get what quadrant a position is in for our rectangular grid
   *
   * 0 1
   * 2 3
   *
   * Problem states that the middle row/col are not part of any quadrant
   * so return -1 for those
   * @param pos
   * @return int - 0, 1, 2, 3, -1
   */
  int get_quadrant(Position pos) {
    if (pos.x < width / 2 && pos.y < height / 2) {
      return 0;
    } else if (pos.x > width / 2 && pos.y < height / 2) {
      return 1;
    } else if (pos.x < width / 2 && pos.y > height / 2) {
      return 2;
    } else if (pos.x > width / 2 && pos.y > height / 2) {
      return 3;
    }
    return -1;
  }
  /**
   * @brief Robots move in a straight line given by their velocity. Move them
   * `num_moves`
   *
   * Something fun about the board they live on though is that they wrap around
   * from the edges. So if a robot were to move past the right edge, it would
   * wrap around to the left edge. And if it were to move past the bottom edge,
   * it would wrap around to the top edge and so on.
   *
   * @param robot - Robot to simulate movement for
   * @param num_moves - Number of moves to simulate
   * @return Robot - Robot after moving `num_moves`
   */
  Robot move_robot(Robot robot, int num_moves) {
    robot.pos.x = (robot.pos.x + num_moves * robot.vel_x) % width;
    robot.pos.x = robot.pos.x >= 0 ? robot.pos.x : robot.pos.x + width;
    robot.pos.y = (robot.pos.y + num_moves * robot.vel_y) % height;
    robot.pos.y = robot.pos.y >= 0 ? robot.pos.y : robot.pos.y + height;
    return robot;
  }
};
std::regex robot_regex("p=(\\d+),(\\d+) v=(-?\\d+),(-?\\d+)");
Robot parse_robot(const std::string &line) {
  Robot robot;
  std::smatch matches;
  if (!std::regex_match(line, matches, robot_regex)) {
    throw std::runtime_error("Invalid input file, expected robot position and "
                             "velocity");
  }
  robot.pos.x = std::stoi(matches[1]);
  robot.pos.y = std::stoi(matches[2]);
  robot.vel_x = std::stoi(matches[3]);
  robot.vel_y = std::stoi(matches[4]);
  return robot;
}
// Add operator<< overload for Robot
inline std::ostream &operator<<(std::ostream &os, const Robot &robot) {
  return os << "Robot(pos=<" << robot.pos.x << ", " << robot.pos.y << ">, vel=<"
            << robot.vel_x << ", " << robot.vel_y << ">)";
}
std::pair<float, float> variance(const std::vector<Robot> &robots) {
  float mean_x = 0, mean_y = 0;
  for (auto robot : robots) {
    mean_x += robot.pos.x;
    mean_y += robot.pos.y;
  }
  mean_x /= robots.size();
  mean_y /= robots.size();
  float variance_x = 0, variance_y = 0;
  for (auto robot : robots) {
    variance_x += (robot.pos.x - mean_x) * (robot.pos.x - mean_x);
    variance_y += (robot.pos.y - mean_y) * (robot.pos.y - mean_y);
  }
  variance_x /= robots.size();
  variance_y /= robots.size();
  return {variance_x, variance_y};
}
}; // namespace day14
namespace aoc {
using namespace day14;

void solve_day14_part1(const std::string &input_path) {
  std::unordered_map<int, int> robots_per_quadrant;
  Day14Grid grid(grid_width, grid_height);
  for (LineIterator it(input_path); it != LineIterator::end(); ++it) {
    Robot robot = parse_robot(*it);
    int quadrant = grid.get_quadrant(grid.move_robot(robot, 100).pos);
    if (quadrant != -1) {
      robots_per_quadrant[quadrant]++;
    }
  }
  // compute safety score
  int safety_score = 1;
  for (auto &entry : robots_per_quadrant) {
    safety_score *= entry.second;
  }
  std::cout << "\nPart 1: " << safety_score << std::endl;
}
long long modular_inverse(long long value, long long modulus) {
  long long original_modulus = modulus;
  long long previous_x = 0, current_x = 1;

  if (modulus == 1) {
    return 0;
  }

  while (value > 1) {
    long long quotient = value / modulus;
    long long temporary = modulus;

    modulus = value % modulus;
    value = temporary;
    temporary = previous_x;

    previous_x = current_x - quotient * previous_x;
    current_x = temporary;
  }

  if (current_x < 0) {
    current_x += original_modulus;
  }

  return current_x;
}
void solve_day14_part2(const std::string &input_path) {
  Day14Grid grid(grid_width, grid_height);
  std::vector<Robot> robots = {};
  for (LineIterator it(input_path); it != LineIterator::end(); ++it) {
    robots.push_back(parse_robot(*it));
  }
  int best_time_x = 0, best_time_y = 0;
  float best_variance_x = std::numeric_limits<float>::max(),
        best_variance_y = std::numeric_limits<float>::max();
  for (int i = 0; i < std::max(grid.height, grid.width); i++) {
    std::vector<Robot> new_robots = {};
    for (Robot &robot : robots) {
      new_robots.push_back(grid.move_robot(robot, i));
    }
    float variance_x, variance_y;
    std::tie(variance_x, variance_y) = variance(new_robots);
    // std::cout << "time: " << i << " variance_x: " << variance_x
    //           << " variance_y: " << variance_y << std::endl;
    if (variance_x < best_variance_x) {
      best_variance_x = variance_x;
      best_time_x = i;
    }
    if (variance_y < best_variance_y) {
      best_variance_y = variance_y;
      best_time_y = i;
    }
  }
  // chinese remainder theorem.
  // thanks to
  // https://www.reddit.com/r/adventofcode/comments/1he0asr/comment/m1zzfsh for
  // calling out this cool method
  // best_x_time + ((inverse_pow(W) % H)*(best_y_time - best_x_time )) % H)*W))
  long long result = best_time_x + ((modular_inverse(grid_width, grid_height) *
                                     (best_time_y - best_time_x)) %
                                    grid_height) *
                                       grid_width;

  std::cout << "\nPart 2: " << result << std::endl;
}
}; // namespace aoc
