#pragma once

#include <fstream>
#include <string>

class LineIterator {
public:
  // Constructors
  LineIterator(const std::string &filename);
  LineIterator(); // End iterator constructor

  // Delete copy operations
  LineIterator(const LineIterator &) = delete;
  LineIterator &operator=(const LineIterator &) = delete;

  // Add move operations
  LineIterator(LineIterator &&) = default;
  LineIterator &operator=(LineIterator &&) = default;

  // Iterator operations
  std::string const &operator*() const;
  LineIterator &operator++();
  bool operator!=(const LineIterator &other) const;
  bool operator==(const LineIterator &other) const;

  // Begin and end functions
  LineIterator &begin();
  static LineIterator end();

private:
  std::ifstream file;
  std::string current_line;
  bool is_valid;
};
