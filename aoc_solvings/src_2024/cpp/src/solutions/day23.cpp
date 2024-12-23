#include "day23.hpp"
#include "utils/line_reader.hpp"
#include <algorithm>
#include <iostream>
#include <iterator>
#include <ostream>
#include <string>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

template <>
struct std::hash<std::tuple<std::string, std::string, std::string>> {
  size_t
  operator()(const std::tuple<std::string, std::string, std::string> &p) const {
    return hash<std::string>()(std::get<0>(p)) ^
           (hash<std::string>()(std::get<1>(p)) << 1) ^
           (hash<std::string>()(std::get<2>(p)) << 2);
  }
};
inline std::ostream &operator<<(std::ostream &os,
                                const std::unordered_set<std::string> &set) {
  for (std::string item : set) {
    os << item << "-";
  }
  return os;
}
namespace day23 {
std::unordered_map<std::string, std::vector<std::string>>
parse_input(const std::string &input_path) {
  LineIterator it(input_path);
  std::unordered_map<std::string, std::vector<std::string>> map;
  while (!(*it).empty()) {
    std::string line = *it;
    std::string first = line.substr(0, 2);
    std::string second = line.substr(3);
    if (!map.contains(first)) {
      map[first] = std::vector<std::string>();
    }
    if (!map.contains(second)) {
      map[second] = std::vector<std::string>();
    }
    map[first].push_back(second);
    map[second].push_back(first);
    ++it;
  }
  return map;
}
std::unordered_set<std::tuple<std::string, std::string, std::string>>
build_player_game(
    const std::unordered_map<std::string, std::vector<std::string>> &map) {
  std::unordered_set<std::tuple<std::string, std::string, std::string>>
      game_triples;
  for (auto [key, connections] : map) {
    if (key[0] != 't') {
      continue;
    }
    for (std::string connection : connections) {
      for (std::string connection2 : map.at(connection)) {
        if (std::find(connections.begin(), connections.end(), connection2) !=
            connections.end()) {
          std::vector<std::string> triple = {key, connection, connection2};
          // make a repeatable sequence of this game
          std::sort(triple.begin(), triple.end());
          game_triples.insert(std::make_tuple(triple[0], triple[1], triple[2]));
        }
      }
    }
  }
  return game_triples;
}
std::vector<std::string> get_largest_group(
    const std::unordered_map<std::string, std::vector<std::string>> &map) {
  std::vector<std::string> keys;
  for (auto [key, _] : map) {
    keys.push_back(key);
  }
  std::sort(keys.begin(), keys.end());
  std::vector<std::unordered_set<std::string>> groups;
  for (auto key : keys) {
    auto connections = map.at(key);
    // for this keys current connections find the best intersecting group
    // best is measured by preserving the size of the group from `groups`
    std::unordered_set<std::string> this_group =
        std::unordered_set<std::string>(connections.begin(), connections.end());
    bool found = false;
    for (auto &group : groups) {
      std::unordered_set<std::string> intersection;
      for (std::string player : group) {
        if (this_group.contains(player)) {
          intersection.insert(player);
        }
      }
      // not sure where the bug is in below, but it doesn't work quite right
      // std::set_intersection(this_group.begin(), this_group.end(),
      // group.begin(),
      //                       group.end(),
      //                       std::inserter(intersection,
      //                       intersection.begin()));
      if (intersection.size() == group.size()) {
        // this group is the best for this key
        // add this key to it
        // and we can stop for this key
        group.insert(key);
        found = true;
        break;
      }
    }
    if (!found) {
      // no best group for this key found, have to create a new group
      std::unordered_set<std::string> new_group;
      new_group.insert(key);
      groups.push_back(new_group);
    }
  }
  size_t best_size = 0;
  std::unordered_set<std::string> best_group;
  for (auto group : groups) {
    if (group.size() > best_size) {
      best_size = group.size();
      best_group = group;
    }
  }
  std::vector<std::string> largest_group(best_group.begin(), best_group.end());
  return largest_group;
}
}; // namespace day23
namespace aoc {
using namespace day23;

void solve_day23_part1(const std::string &input_path) {
  std::unordered_map<std::string, std::vector<std::string>> map =
      parse_input(input_path);
  auto valid_game = build_player_game(map);
  std::cout << "\nPart 1: " << valid_game.size() << std::endl;
}
void solve_day23_part2(const std::string &input_path) {
  std::unordered_map<std::string, std::vector<std::string>> map =
      parse_input(input_path);
  std::vector<std::string> largest_group = get_largest_group(map);
  std::sort(largest_group.begin(), largest_group.end());
  std::string output = "";
  for (std::string player : largest_group) {
    output += player;
    output += ",";
  }
  std::cout << "\nPart 2: " << output << std::endl;
}
}; // namespace aoc
