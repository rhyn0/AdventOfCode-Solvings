#include "line_reader.hpp"
#include <fstream>
#include <string>

// Constructor
LineIterator::LineIterator(const std::string &filename) {
  file.open(filename);
  is_valid = static_cast<bool>(std::getline(file, current_line));
}

// End iterator constructor
LineIterator::LineIterator() : is_valid(false) {}

// Iterator operations
const std::string &LineIterator::operator*() const { return current_line; }

LineIterator &LineIterator::operator++() {
  is_valid = static_cast<bool>(std::getline(file, current_line));
  return *this;
}

bool LineIterator::operator!=(const LineIterator &other) const {
  return is_valid != other.is_valid;
}

// For range-based for loop
LineIterator &LineIterator::begin() { return *this; }
LineIterator LineIterator::end() { return LineIterator(); }
