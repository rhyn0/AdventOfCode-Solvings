#include <chrono>
#include <filesystem>
#include <iomanip>
#include <iostream>
#include <stdexcept>
#include <string>

// Include all day solutions
#include "solutions/day01.hpp"
#include "solutions/day02.hpp"
#include "solutions/day03.hpp"
#include "solutions/day04.hpp"
#include "solutions/day05.hpp"
#include "solutions/day06.hpp"
#include "solutions/day07.hpp"
#include "solutions/day08.hpp"
#include "solutions/day09.hpp"
#include "solutions/day10.hpp"
#include "solutions/day11.hpp"
#include "solutions/day12.hpp"
#include "solutions/day13.hpp"
#include "solutions/day14.hpp"
#include "solutions/day15.hpp"
#include "solutions/day16.hpp"
#include "solutions/day17.hpp"
#include "solutions/day18.hpp"
#include "solutions/day19.hpp"
#include "solutions/day20.hpp"
#include "solutions/day21.hpp"
#include "solutions/day22.hpp"
#include "solutions/day23.hpp"
#include "solutions/day24.hpp"
#include "solutions/day25.hpp"
// ... add more as you implement them

using solve_function = void (*)(const std::string &);

void print_usage() {
  std::cout << "Usage: ./aoc <day> <input_file>\n"
            << "  day: number between 1 and 25\n"
            << "  input_file: path to input file\n"
            << "  --part: optional, specify which part to run (1 or 2). If not "
               "specified, both parts will run.\n";
}

void print_execution_time(const std::chrono::duration<double> &duration,
                          int part) {
  auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(duration);
  std::cout << "\n Part " << part << " Execution time: " << ms.count()
            << "ms\n";
}

int main(int argc, char *argv[]) {
  try {
    // Check arguments
    if (argc < 3) {
      print_usage();
      return 1;
    }

    // Parse day
    int day = std::stoi(argv[1]);
    if (day < 1 || day > 25) {
      throw std::out_of_range("Day must be between 1 and 25");
    }

    // Check if input file exists
    std::string input_path = argv[2];

    // parse optional --part flag
    int part_to_run = 0; // run both parts by default
    for (int i = 3; i < argc; i++) {
      if (std::string(argv[i]) == "--part") {
        if (i + 1 < argc) {
          part_to_run = std::stoi(argv[i + 1]);
          if (part_to_run < 1 || part_to_run > 2) {
            throw std::out_of_range("Part must be between 1 and 2");
          }
        } else {
          throw std::invalid_argument("--part requires an argument");
        }
        break;
      }
    }

    // Map of available solutions
    solve_function solutions[][2] = {
        {aoc::solve_day01_part1, aoc::solve_day01_part2}, // day 1
        {aoc::solve_day02_part1, aoc::solve_day02_part2}, // day 2
        {aoc::solve_day03_part1, aoc::solve_day03_part2}, // day 3
        {aoc::solve_day04_part1, aoc::solve_day04_part2}, // day 4
        {aoc::solve_day05_part1, aoc::solve_day05_part2}, // day 5
        {aoc::solve_day06_part1, aoc::solve_day06_part2}, // day 6
        {aoc::solve_day07_part1, aoc::solve_day07_part2}, // day 7
        {aoc::solve_day08_part1, aoc::solve_day08_part2}, // day 8
        {aoc::solve_day09_part1, aoc::solve_day09_part2}, // day 9
        {aoc::solve_day10_part1, aoc::solve_day10_part2}, // day 10
        {aoc::solve_day11_part1, aoc::solve_day11_part2}, // day 11
        {aoc::solve_day12_part1, aoc::solve_day12_part2}, // day 12
        {aoc::solve_day13_part1, aoc::solve_day13_part2}, // day 13
        {aoc::solve_day14_part1, aoc::solve_day14_part2}, // day 14
        {aoc::solve_day15_part1, aoc::solve_day15_part2}, // day 15
        {aoc::solve_day16_part1, aoc::solve_day16_part2}, // day 16
        {aoc::solve_day17_part1, aoc::solve_day17_part2}, // day 17
        {aoc::solve_day18_part1, aoc::solve_day18_part2}, // day 18
        {aoc::solve_day19_part1, aoc::solve_day19_part2}, // day 19
        {aoc::solve_day20_part1, aoc::solve_day20_part2}, // day 20
        {aoc::solve_day21_part1, aoc::solve_day21_part2}, // day 21
        {aoc::solve_day22_part1, aoc::solve_day22_part2}, // day 22
        {aoc::solve_day23_part1, aoc::solve_day23_part2}, // day 23
        {aoc::solve_day24_part1, aoc::solve_day24_part2}, // day 24
        {aoc::solve_day25_part1, aoc::solve_day25_part2}, // day 24
        // Add more solution functions as you implement them
    };

    // Calculate array size
    const int available_solutions = sizeof(solutions) / sizeof(solutions[0]);

    // Check if solution exists
    if (day > available_solutions) {
      throw std::runtime_error("Solution not implemented yet");
    }

    // Print header
    std::cout << "\n=== Advent of Code 2024 - Day " << day << " ===\n";

    // Run solution and measure time
    if (part_to_run == 0) {
      for (int part = 0; part < 2; part++) {
        auto start = std::chrono::high_resolution_clock::now();
        solutions[day - 1][part](
            input_path); // Arrays are 0-based, so subtract 1
        auto end = std::chrono::high_resolution_clock::now();
        // Print execution time
        print_execution_time(end - start, part + 1);
      }
    } else {
      auto start = std::chrono::high_resolution_clock::now();
      solutions[day - 1][part_to_run - 1](
          input_path); // Arrays are 0-based, so subtract 1
      auto end = std::chrono::high_resolution_clock::now();
      // Print execution time
      print_execution_time(end - start, part_to_run);
    }

  } catch (const std::invalid_argument &e) {
    std::cerr << "Error: Invalid day format " << e.what() << "\n";
    print_usage();
    return 1;
  } catch (const std::out_of_range &e) {
    std::cerr << "Error: " << e.what() << "\n";
    print_usage();
    return 1;
  } catch (const std::runtime_error &e) {
    std::cerr << "Error: " << e.what() << "\n";
    return 1;
  } catch (const std::exception &e) {
    std::cerr << "Unexpected error: " << e.what() << "\n";
    return 1;
  }

  return 0;
}
