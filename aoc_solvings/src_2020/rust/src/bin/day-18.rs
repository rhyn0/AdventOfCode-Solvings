use peg::parser;
use std::env;
use std::iter::Sum;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    println!("Part 1: {}", ProblemStatement::part1(&input_str));
    println!("Part 2: {}", ProblemStatement::part2(&input_str));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

#[derive(Debug, Default, Clone)]
pub struct ProblemStatement {
    // each line is its own expression from input text
    expressions: Vec<Expr>,
}

impl ProblemStatement {
    /// # Panics
    /// Panics if input has characters other than those allowed ('#' or '.')
    #[must_use]
    pub fn parse_input(input: &str, add_precedence: bool) -> Self {
        parser!(
            grammar math_parser() for str {
                // 1 + 2 * 3 + 4 * 5 + 6
                pub rule expression_rules(p: &mut ProblemStatement)
                    = (line(p) newline()*)* ![_]
                rule line(p: &mut ProblemStatement)
                    = i:expression() {p.expressions.push(i)}
                rule expression() -> Expr
                    = first:unit() ts:(whitespace() op:['+' | '*'] whitespace() ts:unit() {(op, ts)})+ {
                        // for every expression, there is a list of terms
                        // the first term is definitely an Operand (number)
                        // for every operator (Add, Mul) we must read the previous term
                        // and the following operand
                        ts.into_iter().fold(first, |acc, (op, next)| {
                            match op {
                                '+' => Expr::Add(vec![acc, next]),
                                '*' => Expr::Mul(vec![acc, next]),
                                _ => unreachable!(),
                            }
                        })
                    }
                rule unit() -> Expr
                    = e:(number() / ("(" a:expression() ")" {a})) {e}
                rule number() -> Expr
                    = e:$(['0'..='9']+) { Expr::Literal(e.parse::<u64>().unwrap())}
                rule whitespace()
                    = [' ' | '\t' | '\r']
                rule newline()
                    = ['\n' | '\r']+
            }
        );
        parser!(
                grammar add_parser() for str {
                    // 1 + 2 * 3 + 4 * 5 + 6
                    pub rule expression_rules(p: &mut ProblemStatement)
                        = (line(p) newline()*)* ![_]
                    rule line(p: &mut ProblemStatement)
                        = i:expression() {p.expressions.push(i)}
                    rule expression() -> Expr
                        = first:expression_plus() ts:(" * " ts:expression_plus() {ts})* {
                            // for every expression, there is a list of terms
                            // the first term is definitely an Operand (number)
                            // for every operator (Add, Mul) we must read the previous term
                            // and the following operand
                            ts.into_iter().fold(first, |acc, next| {
                                Expr::Mul(vec![acc, next])
                            })
                        }
                    rule expression_plus() -> Expr
                        = first:unit() ts:(" + " ts:unit() {ts})* {
                            // for every expression, there is a list of terms
                            // the first term is definitely an Operand (number)
                            // for every operator (Add, Mul) we must read the previous term
                            // and the following operand
                            ts.into_iter().fold(first, |acc, next| {
                                Expr::Add(vec![acc, next])
                            })
                        }
                    rule unit() -> Expr
                        = e:(number() / ("(" a:expression() ")" {a})) {e}
                    rule number() -> Expr
                        = e:$(['0'..='9']+) { Expr::Literal(e.parse::<u64>().unwrap())}
                    rule whitespace()
                        = [' ' | '\t' | '\r']
                    rule newline()
                        = ['\n' | '\r']+
            }
        );
        let mut problem = Self::default();
        if add_precedence {
            add_parser::expression_rules(input, &mut problem).unwrap();
        } else {
            math_parser::expression_rules(input, &mut problem).unwrap();
        }
        problem
    }

    #[must_use]
    /// # Panics
    /// When final value from expression cannot be simplified to a literal value
    pub fn part1(input: &str) -> usize {
        Self::parse_input(input, false)
            .expressions
            .into_iter()
            .map(|x| x.reduce())
            .sum::<Expr>()
            .try_into()
            .unwrap()
    }

    #[must_use]
    /// # Panics
    /// When final value from expression cannot be simplified to a literal value
    pub fn part2(input: &str) -> usize {
        Self::parse_input(input, true)
            .expressions
            .into_iter()
            .map(|x| x.reduce())
            .sum::<Expr>()
            .try_into()
            .unwrap()
    }
}

// An expression is a list of terms, separated by '+' or '*' or a literal value
#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Literal(u64),
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
}

impl Expr {
    pub fn reduce(&self) -> Self {
        match self {
            &Self::Literal(x) => Self::Literal(x),
            Self::Add(items) => {
                if let Some((indx, nested_item)) =
                    // if add has a nested add, we can reduce it to a single add
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
                // otherwise we should try to reduce the terms which are
                // made up of Literals and Mul
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
}

#[allow(clippy::fallible_impl_from)]
impl From<Expr> for usize {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Literal(x) => Self::try_from(x).unwrap(),
            y => match y.reduce() {
                Expr::Literal(x) => Self::try_from(x).unwrap(),
                _ => unreachable!(),
            },
        }
    }
}

impl Sum for Expr {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::Literal(0), |acc, item| {
            Self::Add(vec![acc, item]).reduce()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/test-18.txt";

    #[test]
    fn test_parser() {
        let problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()), false);
        assert_eq!(problem.expressions.len(), 2);
        assert!(matches!(problem.expressions[0], Expr::Add(_)));
        assert!(matches!(problem.expressions[1], Expr::Add(_)));
    }

    #[test]
    fn test_reduce_example1() {
        let problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()), false);
        assert_eq!(problem.expressions[0].clone().reduce(), Expr::Literal(71));
    }

    #[test]
    fn test_reduce_example2() {
        let problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()), false);
        assert_eq!(problem.expressions[1].clone().reduce(), Expr::Literal(51));
    }

    #[test]
    fn test_reduce_add_prec_example1() {
        let problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()), true);
        assert_eq!(problem.expressions[0].clone().reduce(), Expr::Literal(231));
    }

    #[test]
    fn test_reduce_add_prec_example2() {
        let problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()), true);
        assert_eq!(problem.expressions[1].clone().reduce(), Expr::Literal(51));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(
            ProblemStatement::part1(&get_input(&TEST_FILE.to_string())),
            122
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            ProblemStatement::part2(&get_input(&TEST_FILE.to_string())),
            282
        );
    }
}
