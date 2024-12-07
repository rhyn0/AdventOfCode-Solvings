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
// ... add more as you implement them

using solve_function = void (*)(const std::string &);

void print_usage() {
  std::cout << "Usage: ./aoc <day> <input_file>\n"
            << "  day: number between 1 and 25\n"
            << "  input_file: path to input file\n";
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
    if (argc != 3) {
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
    // Map of available solutions
    solve_function solutions[][2] = {
        {aoc::solve_day01_part1, aoc::solve_day01_part2}, // day 1
        {aoc::solve_day02_part1, aoc::solve_day02_part2}, // day 2
        {aoc::solve_day03_part1, aoc::solve_day03_part2}, // day 3
        {aoc::solve_day04_part1, aoc::solve_day04_part2}, // day 4
        {aoc::solve_day05_part1, aoc::solve_day05_part2}, // day 4
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
    for (int part = 0; part < 2; part++) {
      auto start = std::chrono::high_resolution_clock::now();
      solutions[day - 1][part](input_path); // Arrays are 0-based, so subtract 1
      auto end = std::chrono::high_resolution_clock::now();
      // Print execution time
      print_execution_time(end - start, part + 1);
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
