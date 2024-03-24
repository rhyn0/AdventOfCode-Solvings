# Advent of Code 2023 in Rust

I only recently started writing Rust code and still have a lot to learn. What are good packages to use for common problems? How to make reusable objects or Traits more generic to their use case.

## Learnings

Really glad I got this done, albeit extremely late compared to the normal December 2023 finish. It forced me to understand the strict parts of Rust (borrow checker). I do wish I came up with some better generic algorithmic approaches - rather than ones that might only work on the example problem and my input.

## References

*Incomplete List of Helpful Insights*

- Day 25 - [this comment](https://www.reddit.com/r/adventofcode/comments/18qbsxs/comment/ketzp94/) just made things so plain simple. I was experimenting with [Karger's Algorithm](https://en.wikipedia.org/wiki/Karger%27s_algorithm) but couldn't envision how this would approach the problem properly. Most other people all commented about how only 3 edges need to be cut (meaning Karger's would just output 3, except for Failure) and then I need to figure traverse both components of the graph.
