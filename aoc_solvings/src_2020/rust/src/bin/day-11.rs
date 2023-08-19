use im::Vector;
use itertools::Itertools;
use std::cmp::PartialEq;
use std::env;
use std::fmt;
use std::iter::Extend;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_bytes = get_input(&args[1]);
    let inputs = parse_input(&input_bytes);
    dbg!(&inputs.size);
    dbg!(&inputs);
    println!("Part 1: {}", part1(inputs.clone()));
    println!("Part 2: {}", part2(inputs));
}

fn get_input(input_file: &String) -> Vec<u8> {
    std::fs::read_to_string(input_file)
        .unwrap()
        .as_bytes()
        .to_vec()
}

fn parse_input(input: &[u8]) -> Map<Tile> {
    Map::<Tile>::parse(input)
}

fn part1(map: Map<Tile>) -> usize {
    let stable_map = map.last();
    dbg!(&stable_map);
    count_occupied_seats(&stable_map)
}

fn part2(map: Map<Tile>) -> usize {
    let stable_map = map.last_visible();
    dbg!(&stable_map);
    count_occupied_seats(&stable_map)
}

fn count_occupied_seats(map: &Map<Tile>) -> usize {
    map.iter()
        .filter(|p| matches!(p.1, Tile::OccupiedSeat))
        .count()
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Vec2D {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, PartialEq, Default)]
enum Tile {
    #[default]
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Debug)]
struct Positioned<T>(Vec2D, T);

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr_char = match self {
            Self::EmptySeat => 'L',
            Self::Floor => '.',
            Self::OccupiedSeat => '#',
        };
        write!(f, "{repr_char}")
    }
}

impl Tile {
    // change in tile between rounds
    fn next<I>(self, neighbors: I, neighbor_limit: Option<usize>) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let occupied_limit = neighbor_limit.unwrap_or(3);
        match self {
            // always a floor
            Self::Floor => Self::Floor,
            // only becomes occupied if no other seats taken around it
            Self::EmptySeat => match neighbors
                .filter(|t| matches!(t, Self::OccupiedSeat))
                .count()
            {
                0 => Self::OccupiedSeat,
                _ => Self::EmptySeat,
            },
            // occupied to unoccupied if 4 or more neighbors
            Self::OccupiedSeat => {
                if (0..=occupied_limit).contains(
                    &neighbors
                        .filter(|t| matches!(t, Self::OccupiedSeat))
                        .count(),
                ) {
                    Self::OccupiedSeat
                } else {
                    Self::EmptySeat
                }
            }
        }
    }
}

#[derive(PartialEq, Clone)]
struct Map<T>
where
    T: Clone,
{
    size: Vec2D,
    tiles: Vector<T>,
}

impl<T> Map<T>
where
    T: Default + Clone,
{
    fn new(size: Vec2D) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles).map(|_| Default::default()).collect(),
        }
    }
}

impl<T> fmt::Debug for Map<T>
where
    T: fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                // use debug string, impl Debug for Tile
                write!(f, "{:?}", self.get(Vec2D { x, y }).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<A> Extend<Positioned<A>> for Map<A>
where
    A: Copy,
{
    fn extend<T: IntoIterator<Item = Positioned<A>>>(&mut self, iter: T) {
        for Positioned(pos, t) in iter {
            self.set(pos, t);
        }
    }
}

impl<T> Map<T>
where
    T: Copy,
{
    // function to turn 2D coordinate into 1D index
    fn index(&self, point: Vec2D) -> Option<usize> {
        #[allow(clippy::cast_sign_loss)]
        if (0..self.size.x).contains(&point.x) && (0..self.size.y).contains(&point.y) {
            Some((point.x + point.y * self.size.x).try_into().unwrap())
        } else {
            None
        }
    }
    // set tile type at 2D position
    fn set(&mut self, point: Vec2D, tile: T) {
        if let Some(index) = self.index(point) {
            self.tiles[index] = tile;
        }
    }
    #[allow(clippy::unused_self)]
    fn neighbor_positions(&self, pos: Vec2D) -> impl Iterator<Item = Vec2D> {
        (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .filter_map(move |(dx, dy)| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    Some(Vec2D {
                        x: pos.x + dx,
                        y: pos.y + dy,
                    })
                }
            })
    }
    // what is the value at 2D position
    fn get(&self, point: Vec2D) -> Option<T> {
        self.index(point).map(|idx| self.tiles[idx])
    }
    // the positions are helpful but would be faster to just give the actual T value
    // '_ = lifetime of iterator is only as long as Map is borrowed
    fn neighbor_tiles(&self, point: Vec2D) -> impl Iterator<Item = T> + '_ {
        self.neighbor_positions(point)
            .filter_map(move |v| self.get(v))
    }
    // build a position then copy the tile at that position
    fn iter(&self) -> impl Iterator<Item = Positioned<T>> + '_ {
        (0..self.size.y).flat_map(move |y| {
            (0..self.size.x).map(move |x| {
                let pos = Vec2D { x, y };
                Positioned(pos, self.get(pos).unwrap())
            })
        })
    }
}

impl Map<Tile> {
    fn parse(input: &[u8]) -> Self {
        // read in byte version on input file
        let mut curr_columns = 0;
        let mut max_columns = 0; // depending on input, might end with newline
        let mut rows = 1;
        for &c in input.iter() {
            if c == b'\n' {
                rows += 1;
                max_columns = std::cmp::max(max_columns, curr_columns);
                curr_columns = 0;
            } else {
                curr_columns += 1;
            }
        }
        let mut new_iter = input.iter().copied();
        let mut map = Self::new(Vec2D {
            x: max_columns,
            y: rows,
        });
        for y in 0..map.size.y {
            for x in 0..map.size.x {
                let tile = match new_iter.next() {
                    Some(b'L') => Tile::EmptySeat,
                    Some(b'#') => Tile::OccupiedSeat,
                    Some(b'.') => Tile::Floor,
                    // None occurs when file has extra LF at end
                    None => break,
                    c => panic!("Expected tiles of type 'L', '#', or '.' but got {c:?}"),
                };
                map.set(Vec2D { x, y }, tile);
            }
            // skip the newline character
            new_iter.next();
        }
        map
    }
    fn next(&self) -> Self {
        let mut result = Self::new(self.size);
        result.extend(self.iter().map(|Positioned(pos, tile)| {
            Positioned(pos, tile.next(self.neighbor_tiles(pos), None))
        }));
        result
    }
    /// Return the next iteration of the map, based on visible chairs
    fn next_visible(&self) -> Self {
        let mut result = Self::new(self.size);
        result.extend(self.iter().map(|Positioned(pos, tile)| {
            Positioned(pos, tile.next(self.visible_tiles(pos), Some(4)))
        }));
        result
    }
    fn last(self) -> Self {
        itertools::iterate(self, Self::next)
            .tuple_windows()
            .find_map(|(prev, next)| if prev == next { Some(next) } else { None })
            .unwrap()
    }
    /// Return the stable state of the map, based on visible chairs
    fn last_visible(self) -> Self {
        itertools::iterate(self, Self::next_visible)
            .tuple_windows()
            .find_map(|(prev, next)| if prev == next { Some(next) } else { None })
            .unwrap()
    }
    /// Iterator over the chairs available in 4 cardinal and 4 ordinal directions
    fn visible_tiles(&self, point: Vec2D) -> impl Iterator<Item = Tile> + '_ {
        // find first visible seat in all 4 primary cardinal directions
        // and 4 ordinal directions from point
        (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .filter_map(move |(dx, dy)| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    Some(
                        itertools::iterate(point, move |v| Vec2D {
                            x: v.x + dx,
                            y: v.y + dy,
                        })
                        .skip(1)
                        .map(|v| self.index(v))
                        .while_some()
                        .filter_map(move |idx| match self.tiles[idx] {
                            Tile::Floor => None,
                            seat => Some(seat),
                        })
                        .take(1),
                    )
                }
            })
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_FILE: &str = "inputs/test-11.txt";

    #[test]
    fn test_neighbor_positions() {
        use std::collections::HashSet;

        let test_map = Map::<()>::new(Vec2D { x: 3, y: 3 });
        let positions: HashSet<_> = test_map
            .neighbor_positions(Vec2D { x: 1, y: 1 })
            .map(|v| (v.x, v.y))
            .collect();
        for p in &[
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2), // 8 entries
        ] {
            assert!(positions.contains(p));
        }
    }

    #[test]
    fn test_map_size_empty_file() {
        let input = parse_input(&"".to_string().as_bytes().to_vec());
        assert_eq!(input.size, Vec2D { x: 0, y: 1 });
    }
    #[test]
    fn test_map_size_extra_newline() {
        let input = parse_input(&"..\n".to_string().as_bytes().to_vec());
        assert_eq!(input.size, Vec2D { x: 2, y: 2 });
        // it is fine to have an extra row of Tile::Floor
        // since Floor has no effect on seating
        assert_eq!(format!("{:?}", input), "..\n..\n");
    }

    #[test]
    fn test_map_round_next() {
        let input = parse_input(&get_input(&TEST_FILE.to_string()));

        // given what after one round iteration, the new seat layout
        assert_eq!(
            format!("{:?}", input.next()),
            indoc! {"
                #.##.##.##
                #######.##
                #.#.#..#..
                ####.##.##
                #.##.##.##
                #.#####.##
                ..#.#.....
                ##########
                #.######.#
                #.#####.##
            "}
        );
    }

    #[test]
    fn test_visible_tiles_example1() {
        let text_input = indoc! {"
            .......#.
            ...#.....
            .#.......
            .........
            ..#L....#
            ....#....
            .........
            #........
            ...#.....
            "}
        .trim()
        .as_bytes()
        .to_vec();
        let map = Map::<Tile>::parse(&text_input);
        let open_seat = Vec2D { x: 3, y: 4 };
        assert_eq!(map.visible_tiles(open_seat).count(), 8);
        assert_eq!(
            map.visible_tiles(open_seat)
                .filter(|tile| matches!(tile, Tile::OccupiedSeat))
                .count(),
            8
        );
        // but also make sure this func works for other spots
        // one in same row, one in same column - from top left corner
        assert_eq!(map.visible_tiles(Vec2D { x: 0, y: 0 }).count(), 2);
    }

    #[test]
    fn test_visible_tiles_example2() {
        let text_input = indoc! {"
            .............
            .L.L.#.#.#.#.
            .............
            "}
        .trim()
        .as_bytes()
        .to_vec();
        let map = Map::<Tile>::parse(&text_input);
        let left_open_seat = Vec2D { x: 1, y: 1 };
        assert_eq!(map.visible_tiles(left_open_seat).count(), 1);
        assert_eq!(
            map.visible_tiles(left_open_seat)
                .filter(|t| matches!(t, Tile::EmptySeat))
                .count(),
            1
        );
    }

    #[test]
    fn test_visible_tiles_example3() {
        let text_input = indoc! {"
            .##.##.
            #.#.#.#
            ##...##
            ...L...
            ##...##
            #.#.#.#
            .##.##.
            "}
        .trim()
        .as_bytes()
        .to_vec();
        let map = Map::<Tile>::parse(&text_input);
        let open_seat = Vec2D { x: 3, y: 3 };
        assert_eq!(map.visible_tiles(open_seat).count(), 0);
    }

    #[test]
    fn test_visible_iteration() {
        let input = parse_input(&get_input(&TEST_FILE.to_string()));
        assert_eq!(
            format!("{:?}", input.next_visible()),
            indoc! {"
                #.##.##.##
                #######.##
                #.#.#..#..
                ####.##.##
                #.##.##.##
                #.#####.##
                ..#.#.....
                ##########
                #.######.#
                #.#####.##
            "}
        )
    }

    #[test]
    fn correct_part1() {
        let input = parse_input(&get_input(&TEST_FILE.to_string()));

        assert_eq!(part1(input), 37);
    }

    #[test]
    fn correct_part2() {
        let input = parse_input(&get_input(&TEST_FILE.to_string()));

        assert_eq!(part2(input), 26);
    }
}
