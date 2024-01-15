# Advent of Code

Accumulation of many [Advent of Code](https://adventofcode.com/) solvings in various languages.

## Code

### Getting inputs

I use this helpful Python [package](https://github.com/wimglenn/advent-of-code-data) to get inputs either directly from import statements in Python, or to save them to files via a helpful CLI that is included.

Since each user's input is semi-unique for Advent of Code, it requires your SESSION TOKEN from the website to make requests on your behalf. The `advent-of-code-data` package does a great job exlpaining how to do that and setting it as config.
The token is long lived but it will need to be refreshed at times. I store mine according to the `~/.config/aocd/token` file format.
