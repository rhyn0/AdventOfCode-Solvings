# Advent of Code 2023 in C++

I used to write C code a lot for learning as I was able to inspect the byte code and dive deeper on memory allocation and other low level topics. I explored C++ a bit but didn't make a lot of projects using it. Just want to do a quick refresher on this.

## Learnings

TBD - haven't gotten through a lot

## Usage

After building the binary - see [**Building**](#building) - you can run it with your desired day and desired input.

Help message:

```plaintext
Usage: ./aoc <day> <input_file>
  day: number between 1 and 25
  input_file: path to input file
```

## Code and Content

This is a single entry point C++ implementation for solving Advent of Code. As is stated in the [Reddit Wiki](https://www.reddit.com/r/adventofcode/wiki/faqs/copyright/inputs/) this section (got to clean the others) will not share the "personal" input problems. But I do make copies of the sample data inside here.

Sample input files are saved as `sample_inputs/dayXX.txt`.
The exact problem input data is not committed but inside the folder called `inputs`. If you clone this repository and follow the commands I copy-paste into here, you can follow suit and put your data there too.

### Building

```shell
mkdir build
cd build
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON .. && cmake --build .
```

Need the extra option flag only *once* to create the `compile_commands.json` file. This is important for `clang-tidy` in [Linting](#linting) later.

### Style

This folder section will use `clang-format` for formatting. Configuration is close to LLVM style and saved in `.clang-format`. To run the formatter

```shell
clang-format --style=file --Werror -i src/**/*.{c,h}pp
```

### Linting

This folder section will use `clang-tidy` for linting. Configuration is close to default config and saved in `.clang-tidy`. To run the linter

```shell
clang-tidy -p build src/**/*.{c,h}pp
```

## References

*Incomplete List of Helpful Insights*
