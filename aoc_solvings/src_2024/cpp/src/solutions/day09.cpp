#include "day09.hpp"
#include "utils/input_reader.hpp"
#include <iostream>
#include <string>
#include <vector>

namespace day09 {
class FilePosition {
public:
  int width;
  int file_id;
  FilePosition(int w, int f) {
    width = w;
    file_id = f;
  };
};
bool is_file(size_t idx) { return idx % 2 == 0; }
long file_checksum(int file_id, int idx) { return file_id * idx; }
std::vector<FilePosition> parse_numbers(const std::string &input_path) {
  std::vector<FilePosition> numbers;
  int idx = 0;
  int file_id = 0;
  for (char c : FileReader::readFile(input_path)) {
    if (c == '\n') {
      break;
    }
    numbers.push_back(FilePosition(c - '0', is_file(idx) ? file_id : -1));
    if (is_file(idx)) {
      file_id++;
    }
    idx++;
  }
  return numbers;
}
} // namespace day09
namespace aoc {
using namespace day09;

void solve_day09_part1(const std::string &input_path) {
  size_t left, right;
  std::vector<FilePosition> files = day09::parse_numbers(input_path);
  left = 0;
  right = files.size() - 1;
  if (files[right].file_id == -1) {
    right--;
  }
  int curr_idx = 0;
  long running_sum = 0;
  int remaining_from_right = files[right].width;
  while (left < right) {
    // every loop we move from idx 0 to max idx
    // when we encounter a file, we add it to the running sum
    // if we encounter a blank, we move the values from the right to there
    FilePosition curr = files[left];
    // we have a file
    if (curr.file_id != -1) {
      for (int i = 0; i < curr.width; i++) {
        running_sum += day09::file_checksum(curr.file_id, curr_idx + i);
      }
    }
    // we have a blank, so we have to use the file id on the right
    // which has two cases,
    // 1. we have a file that has been somewhat moved and thus remainder
    // 2. we have a fresh file, that has not been moved at all yet
    else {
      for (int i = 0; i < curr.width; i++) {
        if (remaining_from_right == 0) {
          // right always points to a file
          right -= 2;
          remaining_from_right = files[right].width;
        }
        running_sum += day09::file_checksum(files[right].file_id, curr_idx + i);
        remaining_from_right--;
      }
    }
    curr_idx += curr.width;
    left++;
  }
  // possibly, we have left == right, and there are remainders
  // this means there are blocks of a file that are still unaccounted for
  // we need to add them to the running sum
  for (int i = 0; i < remaining_from_right; i++) {
    running_sum += day09::file_checksum(files[right].file_id, curr_idx + i);
  }
  std::cout << "\nPart 1: " << running_sum << std::endl;
}
void solve_day09_part2(const std::string &input_path) {
  size_t left;
  std::vector<FilePosition> files = day09::parse_numbers(input_path);
  left = 0;
  int curr_idx = 0;
  long running_sum = 0;
  while (left < files.size()) {
    // every loop we move from idx 0 to max idx
    // when we encounter a file, we add it to the running sum
    // if we encounter a blank, we attempt to find a file that can completely
    // move to fill that space. The chosen file is chosen in descending file_id
    // order file width must be less than or equal to the blank width
    FilePosition curr = files[left];
    // we have a file
    if (curr.file_id != -1) {
      for (int i = 0; i < curr.width; i++) {
        running_sum += day09::file_checksum(curr.file_id, curr_idx + i);
      }
      curr_idx += curr.width;
    }
    // we have a blank, so we have to use the file id on the right
    else {
      // until we reach our current position
      // search for a file that is less than or equal to the width of the blank
      int avail_space = curr.width;
      for (size_t right = files.size() - 1; avail_space > 0 && left < right;
           right--) {
        if (files[right].file_id == -1) {
          continue;
        }
        if (files[right].width <= avail_space) {
          for (int i = 0; i < files[right].width; i++) {
            running_sum +=
                day09::file_checksum(files[right].file_id, curr_idx + i);
          }
          // this file has been moved and used, mark this block as empty
          files[right].file_id = -1;
          avail_space -= files[right].width;
          curr_idx += files[right].width;
        }
      }
      // if there is any avail space left, we couldn't fill with a file
      // so we need to move past the blank
      curr_idx += avail_space;
    }
    left++;
  }

  std::cout << "\nPart 2: " << running_sum << std::endl;
}
} // namespace aoc
