use itertools::Itertools;
use std::{env, fmt::Debug};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let problem = parse_input(&input_str);
    println!("Part 1: {}", problem.part1());
    println!("Part 2: {}", problem.part2());
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn parse_input(input: &str) -> ProblemStatement {
    let mut line_iter = input.trim().lines();
    let min_time: usize = line_iter.next().unwrap().trim().parse().unwrap();
    // to accommodate the keeping of 'x' buses, use enumeration
    ProblemStatement {
        my_departure_time: min_time,
        buses: line_iter
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .filter_map(|(idx, val)| match val {
                "x" => None,
                x => Some(Bus::new(x.parse::<usize>().unwrap(), idx, None)),
            })
            .collect_vec(),
    }
}

#[derive(Clone)]
struct ProblemStatement {
    my_departure_time: usize,
    buses: Vec<Bus>,
}

impl ProblemStatement {
    fn part1(&self) -> usize {
        let all_bus_next_times = self
            .buses
            .iter()
            .map(|bus| bus.next(self.my_departure_time))
            .sorted_by_key(|bus| bus.next_arrive)
            .collect_vec();
        let next_bus = all_bus_next_times[0];
        waiting_score(next_bus.next_arrive - self.my_departure_time, next_bus)
    }

    fn part2(&self) -> i64 {
        // return earliest timestamp that all buses leave in the order of the given array
        // having an X in the sequence gives a gap minute
        solve_lincon_system(self.buses.iter().map(|bus| LinearCongruence {
            lhs: Expr::Var('x'),
            rhs: Expr::Literal((bus.id as i64 - bus.time_offset as i64).rem_euclid(bus.id as _)),
            modulo: bus.id.try_into().unwrap(),
        }))
    }
}

#[derive(Copy, Clone, Debug)]
struct Bus {
    id: usize,
    time_offset: usize,
    next_arrive: usize,
}

#[derive(Copy, Clone)]
struct WrongOffset<'a> {
    prev_bus: &'a Bus,
    curr_bus: &'a Bus,
    expected_offset: usize,
    actual_offset: usize,
}

#[derive(Clone, Eq, PartialEq)]
enum Expr {
    Literal(i64),
    Var(char),
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
}

#[derive(Clone, PartialEq, Eq)]
struct LinearCongruence {
    // mathematical representation for modular math
    lhs: Expr,
    rhs: Expr,
    modulo: u32, // modulo needs to be a positive given our Bus timings
}

#[derive(Debug)]
struct CantSolve(LinearCongruence);

impl std::fmt::Display for CantSolve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for CantSolve {}

impl Expr {
    fn reduce(&self) -> Self {
        match self {
            &Self::Literal(x) => Self::Literal(x),
            Self::Var(c) => Self::Var(*c),
            Self::Add(items) => {
                if let Some((indx, nested_item)) =
                    items.iter().enumerate().find_map(|(idx, ex)| match ex {
                        Self::Add(terms) => Some((idx, terms)),
                        _ => None,
                    })
                {
                    return Self::Add(
                        items
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, item)| if idx == indx { None } else { Some(item) })
                            .chain(nested_item)
                            .cloned()
                            .collect(),
                    )
                    .reduce();
                }
                let (literals, others): (Vec<_>, Vec<_>) = items
                    .iter()
                    .map(Self::reduce)
                    .partition(|i| matches!(i, Self::Literal(_)));
                // if no items, it reduced to zero
                if literals.is_empty() && others.is_empty() {
                    Self::Literal(0)
                } else {
                    let mut terms = others;
                    let sum_literal = literals
                        .into_iter()
                        .map(|x| {
                            if let Self::Literal(x) = x {
                                x
                            } else {
                                unreachable!()
                            }
                        })
                        .sum();
                    if sum_literal != 0 {
                        if terms.is_empty() {
                            return Self::Literal(sum_literal);
                        }

                        terms.insert(0, Self::Literal(sum_literal));
                    }
                    if terms.len() == 1 {
                        terms.pop().unwrap()
                    } else {
                        Self::Add(terms)
                    }
                }
            }
            Self::Mul(items) => {
                // pretty much the same as Add but with multiplication
                let (literals, others): (Vec<_>, Vec<_>) = items
                    .iter()
                    .map(Self::reduce)
                    .partition(|i| matches!(i, Self::Literal(_)));
                // if no items, it reduced to zero
                if literals.is_empty() && others.is_empty() {
                    Self::Literal(1)
                } else {
                    let mut terms = others;
                    let product_literal = literals
                        .into_iter()
                        .map(|x| {
                            if let Self::Literal(x) = x {
                                x
                            } else {
                                unreachable!()
                            }
                        })
                        .product();
                    if product_literal != 1 {
                        if terms.is_empty() {
                            return Self::Literal(product_literal);
                        }
                        terms.insert(0, Self::Literal(product_literal));
                    }
                    if terms.len() == 1 {
                        terms.pop().unwrap()
                    } else {
                        Self::Mul(terms)
                    }
                }
            }
        }
    }
    fn modulo(&self, modulo: u32) -> Self {
        match self {
            &Self::Literal(lit) => Self::Literal(lit.rem_euclid(i64::from(modulo))),
            Self::Var(c) => Self::Var(*c),
            Self::Add(_) => self.clone(),
            Self::Mul(items) => Self::Mul(items.iter().map(|x| x.modulo(modulo)).collect()),
        }
    }

    fn substitute(&self, replace: Self) -> Self {
        // use another expression to create system of equations.
        // this expression has Var 'j' which has some equation
        // j = Xk + Y, so we will substitute in that value for j
        match self {
            &Self::Literal(lit) => Self::Literal(lit),
            Self::Var(_) => replace,
            Self::Add(items) => Self::Add(
                items
                    .iter()
                    .cloned()
                    .map(|ex| ex.substitute(replace.clone()))
                    .collect(),
            ),
            Self::Mul(items) => Self::Mul(
                items
                    .iter()
                    .cloned()
                    .map(|ex| ex.substitute(replace.clone()))
                    .collect(),
            ),
        }
    }
    fn add(&self, expr: Self) -> Self {
        match self {
            Self::Add(items) => {
                Self::Add(std::iter::once(expr).chain(items.iter().cloned()).collect())
            }
            _ => Self::Add(vec![expr, self.clone()]),
        }
    }
    fn mul(&self, expr: Self) -> Self {
        match self {
            Self::Mul(items) => {
                Self::Mul(std::iter::once(expr).chain(items.iter().cloned()).collect())
            }
            _ => Self::Mul(vec![expr, self.clone()]),
        }
    }
    fn distribute(&self) -> Self {
        // given expression (X * (Y + z))
        // this is simplified by distributing X
        // to get XY + Xz
        if let Self::Mul(items) = self {
            if let [Self::Literal(outer_x), Self::Add(add_terms)] = &items[..] {
                return Self::Add(
                    add_terms
                        .iter()
                        .map(|t| t.mul(Self::Literal(*outer_x)))
                        .collect(),
                );
            }
        }
        if let Self::Add(items) = self {
            return Self::Add(items.iter().map(Self::distribute).collect());
        }
        self.clone()
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Literal(lit) => write!(f, "{lit}"),
            Self::Var(c) => write!(f, "{c}"),
            Self::Add(terms) => {
                write!(f, "(")?;
                for (i, term) in terms.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{term:?}")?;
                    } else {
                        write!(f, " + {term:?}")?;
                    }
                }
                write!(f, ")")?;
                Ok(())
            }
            Self::Mul(terms) => {
                write!(f, "(")?;
                for (i, term) in terms.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{term:?}")?;
                    } else {
                        write!(f, " * {term:?}")?;
                    }
                }
                write!(f, ")")?;
                Ok(())
            }
        }
    }
}

impl Debug for LinearCongruence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // allow this unicode character, congruence over equality
        write!(f, "{:?} ≡ {:?} (mod {})", self.lhs, self.rhs, self.modulo)
    }
}

impl LinearCongruence {
    fn solve(&self) -> Result<Self, CantSolve> {
        eprintln!("should solve {self:?}");
        if let Expr::Mul(items) = &self.lhs {
            if let [Expr::Literal(lit), Expr::Var(_)] = items[..] {
                let mmi = modular_multiplicative_inverse(lit as _, self.modulo);
                eprintln!("multiplying by mod ult inverse {mmi}");
                return self.mul(Expr::Literal(mmi)).solve();
            }
        }

        // if outer level of expression is a Expr::Add with a literal
        // add the inverse of that value to both sides
        if let Expr::Add(items) = &self.lhs {
            if let Some(lit) = items.iter().find_map(|ex| match ex {
                Expr::Literal(x) => Some(x),
                _ => None,
            }) {
                eprintln!("adding {} on both sides", -*lit);
                return self.add(Expr::Literal(-*lit)).solve();
            }
        }

        if let Expr::Var(_) = &self.lhs {
            return Ok(self.clone());
        }

        Err(CantSolve(self.clone()))
    }

    fn add(&self, expr: Expr) -> Self {
        Self {
            lhs: self.lhs.add(expr.clone()).reduce().modulo(self.modulo),
            rhs: self.rhs.add(expr).reduce().modulo(self.modulo),
            modulo: self.modulo,
        }
    }

    fn mul(&self, expr: Expr) -> Self {
        Self {
            lhs: self.lhs.mul(expr.clone()).reduce().modulo(self.modulo),
            rhs: self.rhs.mul(expr).reduce().modulo(self.modulo),
            modulo: self.modulo,
        }
    }

    fn as_expr(&self, name: char) -> Expr {
        // Linear Congruence of x \equiv 7 (mod 13)
        // turns into expr 13 * j + 7
        match (&self.lhs, &self.rhs) {
            // can only transform isolated variable types into expression
            (Expr::Var(_), &Expr::Literal(remainder)) => Expr::Add(vec![
                Expr::Mul(vec![Expr::Literal(i64::from(self.modulo)), Expr::Var(name)]),
                Expr::Literal(remainder),
            ]),
            _ => panic!(
                "Expected solved congruence (of form `var ≡ literal (mod m)`), but got `{self:?}`",
            ),
        }
    }
    fn substitute(&self, expr: Expr) -> Self {
        Self {
            lhs: self.lhs.substitute(expr.clone()),
            rhs: self.rhs.substitute(expr),
            modulo: self.modulo,
        }
    }
}

impl Bus {
    fn new(id: usize, time_offset: usize, next_arrive: Option<usize>) -> Self {
        Self {
            id,
            time_offset,
            next_arrive: next_arrive.unwrap_or(0),
        }
    }

    fn next(self, min_time: usize) -> Self {
        Self {
            next_arrive: itertools::iterate(self.next_arrive, |&x| x + self.id)
                .find(|&iter_time| iter_time >= min_time)
                .unwrap(),
            ..self
        }
    }
}

impl Debug for WrongOffset<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "expected Bus {} to leave {} minutes after Bus {} but actually was {} minutes",
            self.curr_bus.id, self.expected_offset, self.prev_bus.id, self.actual_offset,
        )
    }
}
impl std::fmt::Display for WrongOffset<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for WrongOffset<'_> {}

const fn waiting_score(wait_time: usize, bus: Bus) -> usize {
    wait_time * bus.id
}

fn solve_lincon_system<I>(mut system: I) -> i64
where
    I: Iterator<Item = LinearCongruence>,
{
    let mut curr_var = b'a';
    let mut next_var = || -> char {
        let res = curr_var as char;
        curr_var += 1;
        res
    };

    let lc = system.next().unwrap();
    let mut x = lc.as_expr(next_var()).reduce();
    println!("currently: {lc:?}");
    println!("x = {x:?}");
    for lc in system {
        println!("currently: {lc:?}");
        x = x
            .substitute(
                lc.substitute(x.clone())
                    .solve()
                    .unwrap()
                    .as_expr(next_var()),
            )
            .distribute()
            .reduce();
        println!("x = {x:?}");
    }
    let x = x.substitute(Expr::Literal(0)).reduce();
    if let Expr::Literal(lit) = x {
        lit
    } else {
        panic!("expected `x` to be a literal but got {x:?}")
    }
}

fn modular_multiplicative_inverse(a: i64, m: u32) -> i64 {
    modular_pow(a, m - 2, i64::from(m))
}

fn modular_pow(x: i64, exp: u32, modulo: i64) -> i64 {
    x.checked_pow(exp).map_or_else(
        || {
            let exp_a = exp / 2;
            let exp_b = exp - exp_a;
            modular_pow(x, exp_a, modulo) * modular_pow(x, exp_b, modulo)
        },
        |x| x,
    ) % modulo
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/test-13.txt";

    #[test]
    fn test_part2_examples() {
        macro_rules! test {
            ($list: literal, $solution: expr) => {
                assert_eq!(parse_input(concat!("0\n", $list, "\n")).part2(), $solution)
            };
        }
        test!("17,x,13,19", 3417);
        test!("67,7,59,61", 754018);
        test!("67,x,7,59,61", 779210);
        test!("67,7,x,59,61", 1261476);
        test!("1789,37,47,1889", 1202161486);
    }

    #[test]
    fn test_expr_reduce() {
        assert_eq!(Expr::Add(vec![]).reduce(), Expr::Literal(0).reduce());

        assert_eq!(
            Expr::Add(vec![Expr::Literal(2), Expr::Literal(3)]).reduce(),
            Expr::Add(vec![Expr::Literal(5)]).reduce(),
        );

        assert_eq!(
            Expr::Add(vec![Expr::Literal(2), Expr::Literal(3), Expr::Literal(5)]).reduce(),
            Expr::Add(vec![Expr::Literal(10)]).reduce(),
        );

        assert_eq!(
            Expr::Add(vec![Expr::Literal(2), Expr::Literal(3), Expr::Var('x')]).reduce(),
            Expr::Add(vec![Expr::Literal(5), Expr::Var('x')]).reduce(),
        );

        assert_eq!(
            Expr::Mul(vec![Expr::Literal(2), Expr::Literal(3), Expr::Var('x')]).reduce(),
            Expr::Mul(vec![Expr::Literal(6), Expr::Var('x')]).reduce(),
        );

        assert_eq!(
            Expr::Mul(vec![
                Expr::Add(vec![Expr::Literal(2), Expr::Literal(3)]),
                Expr::Literal(10),
                Expr::Var('x')
            ])
            .reduce(),
            Expr::Mul(vec![Expr::Literal(50), Expr::Var('x')]).reduce(),
        );
    }

    #[test]
    fn test_nested_add_reduce() {
        assert_eq!(
            Expr::Add(vec![
                Expr::Add(vec![Expr::Literal(2), Expr::Var('x')]),
                Expr::Literal(10),
            ])
            .reduce(),
            Expr::Add(vec![Expr::Literal(12), Expr::Var('x')]).reduce(),
        );
    }

    #[test]
    fn correct_part1() {
        let problem = parse_input(&get_input(&TEST_FILE.to_string()));

        assert_eq!(problem.part1(), 295);
    }

    #[test]
    fn correct_part2() {
        let problem = parse_input(&get_input(&TEST_FILE.to_string()));

        assert_eq!(problem.part2(), 1068781);
    }
}
