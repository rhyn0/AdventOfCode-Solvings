# Advent of Code 2023 in C++

I used to write C code a lot for learning as I was able to inspect the byte code and dive deeper on memory allocation and other low level topics. I explored C++ a bit but didn't make a lot of projects using it. Just want to do a quick refresher on this.

## Learnings

- function naming. Since combining all these solution files could eventually overload a name, make sure to namespace them.
- writing lambda functions with capture lists

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

## Helpful Commands for Debugging

### Day 20

To make sure i generated all the possible cheats for the sample input, I added the following line at the bottom of `day20.cpp` Part 1
```cpp
 std::cout << "BEST CHEAT from: " << key.first.first << ","
              << key.first.second << " to " << key.second.first << ","
              << key.second.second << " saves " << value << " steps\n";
```

Then used the following shell command to group and count the cheats based on the number of steps saved.

```shell
./aoc-rhyn0 20 ../sample_inputs/day20.txt --part 1  | grep -E '^BEST CHEAT' | grep -o -E '\d+ steps' | cut -d' ' -f1 | sort | uniq -c | sort -n -r
```

## References

*Incomplete List of Helpful Insights*

### Day 14 Part 2

- [Reddit](https://www.reddit.com/r/adventofcode/comments/1he0asr/comment/m1zzfsh)
- [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)

### Day 17 Part 2

I really have no idea what im doing here, will have to come back to this one. 1am coding is not cutting this.
HAHA, i love changing data types for my whole codebase at a later time.

### Day 24 Part 2

- [Full Adder Circuit Diagram](https://upload.wikimedia.org/wikipedia/commons/thumb/6/69/Full-adder_logic_diagram.svg/1920px-Full-adder_logic_diagram.svg.png)

Back to the EE days, essentially want to do a bunch of 1 bit full-adders but only use wire names to solve with.
