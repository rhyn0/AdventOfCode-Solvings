#include "day22.hpp"
#include "utils/line_reader.hpp"
#include <algorithm>
#include <array>
#include <iostream>
#include <string>
#include <vector>

#define PRUNE_MOD 16777216
#define ARR_SIZE 19LU * 19LU * 19LU * 19LU

namespace day22 {
typedef unsigned long MONKEY_DATA;
MONKEY_DATA mix(MONKEY_DATA secret, MONKEY_DATA number) {
  return secret ^ number;
}
MONKEY_DATA prune(MONKEY_DATA secret) { return secret % PRUNE_MOD; }
MONKEY_DATA new_secret_step1(MONKEY_DATA secret) {
  return prune(mix(secret, secret * 64));
}
MONKEY_DATA new_secret_step2(MONKEY_DATA secret) {
  return prune(mix(secret, secret / 32));
}
MONKEY_DATA new_secret_step3(MONKEY_DATA secret) {
  return prune(mix(secret, secret * 2048));
}
MONKEY_DATA get_new_secret(MONKEY_DATA secret) {
  return new_secret_step3(new_secret_step2(new_secret_step1(secret)));
}
/**
 * @brief Return the index at which this set of deltas is meant to be stored
 *
 * Since possible values are -9 to 9, we add by 9 to make all positive, giving
 * range 0 to 18.
 *
 * @param window
 * @return unsigned long
 */
unsigned long get_arr_index(std::vector<int> window) {
  size_t len = window.size();
  return (window[len - 4] + 9) * (19LU * 19LU * 19LU) +
         (window[len - 3] + 9) * (19LU * 19LU) + (window[len - 2] + 9) * 19LU +
         (window[len - 1] + 9);
}
}; // namespace day22
namespace aoc {
using namespace day22;

void solve_day22_part1(const std::string &input_path) {
  LineIterator it(input_path);
  long output_sums = 0;
  while (!(*it).empty()) {
    long curr_secret = std::stol(*it);
    for (int i = 0; i < 2000; i++) {
      curr_secret = get_new_secret(curr_secret);
    }
    output_sums += curr_secret;
    ++it;
  }
  std::cout << "\nPart 1: " << output_sums << std::endl;
}
void solve_day22_part2(const std::string &input_path) {
  std::array<int, ARR_SIZE> bananas_for_sequence;
  bananas_for_sequence.fill(0);
  LineIterator it(input_path);
  while (!(*it).empty()) {
    std::array<bool, ARR_SIZE> sequence_seen;
    sequence_seen.fill(false);
    MONKEY_DATA curr_secret = std::stol(*it);
    // price is the one's digit of the secret
    int old_price = (curr_secret) % 10;
    std::vector<int> window_of_deltas;
    for (int i = 0; i < 2000; i++) {
      curr_secret = get_new_secret(curr_secret);
      int new_price = (curr_secret) % 10;
      int diff = new_price - old_price;
      old_price = new_price;
      window_of_deltas.push_back(diff);
      if (window_of_deltas.size() < 4) {
        continue;
      }
      unsigned long index = get_arr_index(window_of_deltas);
      if (sequence_seen[index]) {
        // can't count the same sequence twice, for the same monkey
        continue;
      }
      sequence_seen[index] = true;
      bananas_for_sequence[index] += new_price;
    }
    ++it;
  }
  int max_bananas = *std::max_element(bananas_for_sequence.begin(),
                                      bananas_for_sequence.end());

  std::cout << "\nPart 2: " << max_bananas << std::endl;
}
}; // namespace aoc
