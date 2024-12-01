#include "input_reader.hpp"
#include <fstream>
#include <sstream>
std::string FileReader::readFile(const std::string &filePath) {
  std::ifstream file(filePath);
  if (!file) {
    throw std::runtime_error("Cannot open input file: " + filePath);
  }
  std::stringstream buffer;
  buffer << file.rdbuf();
  return buffer.str();
}
