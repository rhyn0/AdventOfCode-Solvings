#ifndef FILE_READER_H
#define FILE_READER_H

#include <string>

class FileReader {
public:
  static std::string readFile(const std::string &filePath);

private:
  FileReader() = delete; // Prevent instantiation since all methods are static
};

#endif
