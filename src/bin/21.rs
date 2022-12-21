use std::collections::HashMap;

use itertools::Itertools;
use parse_display::{Display, FromStr};

// NO, dont look at this code. Turn around.
// This could've been done much easier by modeling the expressions with a real
// datastructure for expressions, but Part 1 was simple with this one and then
// I was too lazy to change it and just prodded at it until it works.
// Dont look at this.

#[derive(Display, FromStr, Debug, Clone, PartialEq)]
enum Value {
    #[display("{0}")]
    Number(i64),
    #[display("{0}")]
    Variable(String),
}

#[derive(Display, FromStr, Debug, Clone, PartialEq)]
enum Expression {
    #[display("{0} + {1}")]
    Addition(Value, Value),
    #[display("{0} - {1}")]
    Substraction(Value, Value),
    #[display("{0} * {1}")]
    Multiplication(Value, Value),
    #[display("{0} / {1}")]
    Division(Value, Value),
    #[display("{0} = {1}")]
    Equality(Value, Value),
    #[display("{0}")]
    Literal(Value),
}

impl Expression {
    // calculate an Expression if it has no free variables, otherwise return self
    fn calculate(self) -> Expression {
        match self.clone() {
            Expression::Literal(_) => return self,
            Expression::Equality(_, _) => return self,
            Expression::Addition(x, y) => {
                if let Value::Number(l) = x {
                    if let Value::Number(r) = y {
                        return Expression::Literal(Value::Number(l + r));
                    }
                }
            }
            Expression::Substraction(x, y) => {
                if let Value::Number(l) = x {
                    if let Value::Number(r) = y {
                        return Expression::Literal(Value::Number(l - r));
                    }
                }
            }
            Expression::Multiplication(x, y) => {
                if let Value::Number(l) = x {
                    if let Value::Number(r) = y {
                        return Expression::Literal(Value::Number(l * r));
                    }
                }
            }
            Expression::Division(x, y) => {
                if let Value::Number(l) = x {
                    if let Value::Number(r) = y {
                        return Expression::Literal(Value::Number(l / r));
                    }
                }
            }
        }

        self
    }

    // Flip an expression around, so the variables left and right switch
    fn flip(self, id: String) -> (String, Expression) {
        match self.clone() {
            Expression::Literal(_) => (id, self),
            // a = b + 1 -> b = a - 1
            // a = 1 + b -> b = a - 1
            Expression::Addition(x, y) => {
                let mut new_l = x.clone();
                let mut new_r = y.clone();
                let mut new_id = id.to_string();

                if let Value::Variable(l) = x {
                    if let Value::Number(_) = y {
                        new_l = Value::Variable(id);
                        new_r = y;
                        new_id = l;
                    }
                } else if let Value::Variable(l) = y {
                    if let Value::Number(_) = x {
                        new_l = Value::Variable(id);
                        new_r = x;
                        new_id = l;
                    }
                }
                (new_id, Expression::Substraction(new_l, new_r))
            }
            // a = b - 1 -> b = a + 1
            // a = 1 - b -> b = 1 - a
            Expression::Substraction(x, y) => {
                let new_l = x.clone();
                let new_r = y.clone();
                let new_id = id.to_string();

                if let Value::Variable(l) = x {
                    if let Value::Number(r) = y {
                        return (
                            l,
                            Expression::Addition(Value::Variable(id), Value::Number(r)),
                        );
                    }
                } else if let Value::Variable(l) = y {
                    if let Value::Number(r) = x {
                        return (
                            l,
                            Expression::Substraction(Value::Number(r), Value::Variable(id)),
                        );
                    }
                }
                (new_id, Expression::Substraction(new_l, new_r))
            }
            // a = b * 1 -> b = a / 1
            // a = 1 * b -> b = a / 1
            Expression::Multiplication(x, y) => {
                let mut new_l = x.clone();
                let mut new_r = y.clone();
                let mut new_id = id.to_string();

                if let Value::Variable(l) = x {
                    if let Value::Number(_) = y {
                        new_l = Value::Variable(id);
                        new_r = y;
                        new_id = l;
                    }
                } else if let Value::Variable(l) = y {
                    if let Value::Number(_) = x {
                        new_l = Value::Variable(id);
                        new_r = x;
                        new_id = l;
                    }
                }
                (new_id, Expression::Division(new_l, new_r))
            }
            // a = b / 1 -> b = a * 1
            // a = 1 / b -> b = 1 / a
            Expression::Division(x, y) => {
                let new_l = x.clone();
                let new_r = y.clone();
                let new_id = id.to_string();

                if let Value::Variable(l) = x {
                    if let Value::Number(r) = y {
                        return (
                            l,
                            Expression::Multiplication(Value::Variable(id), Value::Number(r)),
                        );
                    }
                } else if let Value::Variable(l) = y {
                    if let Value::Number(r) = x {
                        return (
                            l,
                            Expression::Division(Value::Number(r), Value::Variable(id)),
                        );
                    }
                }
                (new_id, Expression::Substraction(new_l, new_r))
            }
            // a = b == 1 -> b = 1
            // a = 1 == b -> b = 1
            Expression::Equality(x, y) => {
                let mut new_id = id;
                let mut value = 0;
                if let Value::Variable(l) = x {
                    new_id = l;
                } else if let Value::Number(l) = x {
                    value = l;
                }
                if let Value::Variable(r) = y {
                    new_id = r;
                } else if let Value::Number(r) = y {
                    value = r;
                }
                (new_id, Expression::Literal(Value::Number(value)))
            }
        }
    }
}

fn reduce(map: Vec<(String, Expression)>) -> Vec<(String, Expression)> {
    let mut result = vec![];
    let literals: HashMap<String, i64> = map
        .iter()
        .filter_map(|(id, expr)| {
            if let Expression::Literal(Value::Number(x)) = expr {
                Some((id.clone(), *x))
            } else {
                None
            }
        })
        .collect();

    for (id, expr) in map {
        let new_expr = match expr {
            Expression::Literal(_) => None,
            Expression::Addition(x, y) => {
                let mut new_x = x.clone();
                if let Value::Variable(l) = x {
                    if literals.contains_key(l.as_str()) {
                        new_x = Value::Number(literals[l.as_str()]);
                    }
                }
                let mut new_y = y.clone();
                if let Value::Variable(r) = y {
                    if literals.contains_key(r.as_str()) {
                        new_y = Value::Number(literals[r.as_str()]);
                    }
                }
                Some(Expression::Addition(new_x, new_y))
            }
            Expression::Substraction(x, y) => {
                let mut new_x = x.clone();
                if let Value::Variable(l) = x {
                    if literals.contains_key(l.as_str()) {
                        new_x = Value::Number(literals[l.as_str()]);
                    }
                }
                let mut new_y = y.clone();
                if let Value::Variable(r) = y {
                    if literals.contains_key(r.as_str()) {
                        new_y = Value::Number(literals[r.as_str()]);
                    }
                }
                Some(Expression::Substraction(new_x, new_y))
            }
            Expression::Multiplication(x, y) => {
                let mut new_x = x.clone();
                if let Value::Variable(l) = x {
                    if literals.contains_key(l.as_str()) {
                        new_x = Value::Number(literals[l.as_str()]);
                    }
                }
                let mut new_y = y.clone();
                if let Value::Variable(r) = y {
                    if literals.contains_key(r.as_str()) {
                        new_y = Value::Number(literals[r.as_str()]);
                    }
                }
                Some(Expression::Multiplication(new_x, new_y))
            }
            Expression::Division(x, y) => {
                let mut new_x = x.clone();
                if let Value::Variable(l) = x {
                    if literals.contains_key(l.as_str()) {
                        new_x = Value::Number(literals[l.as_str()]);
                    }
                }
                let mut new_y = y.clone();
                if let Value::Variable(r) = y {
                    if literals.contains_key(r.as_str()) {
                        new_y = Value::Number(literals[r.as_str()]);
                    }
                }
                Some(Expression::Division(new_x, new_y))
            }
            Expression::Equality(x, y) => {
                let mut new_x = x.clone();
                if let Value::Variable(l) = x {
                    if literals.contains_key(l.as_str()) {
                        new_x = Value::Number(literals[l.as_str()]);
                    }
                }
                let mut new_y = y.clone();
                if let Value::Variable(r) = y {
                    if literals.contains_key(r.as_str()) {
                        new_y = Value::Number(literals[r.as_str()]);
                    }
                }
                Some(Expression::Equality(new_x, new_y))
            }
        };
        if let Some(new_ex) = new_expr {
            result.push((id, new_ex));
        }
    }

    result
}

pub fn part_one(_input: &str) -> Option<i64> {
    let mut map = _input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(id, expr)| (id.to_string(), expr.parse::<Expression>().unwrap()))
        .collect_vec();

    // reduce it
    while map.len() > 1 {
        map = reduce(map);

        map = map
            .iter()
            .map(|(id, expr)| (id.clone(), expr.clone().calculate()))
            .collect();
    }

    if let Expression::Literal(Value::Number(x)) = map[0].1 {
        Some(x)
    } else {
        None
    }
}

pub fn part_two(_input: &str) -> Option<i64> {
    let mut map = _input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(id, expr)| match id {
            "humn" => (
                id.to_string(),
                Expression::Literal(Value::Variable("awefawef".to_string())),
            ),
            "root" => (
                id.to_string(),
                expr.replace(['+', '-', '*', '/'], "=")
                    .parse::<Expression>()
                    .unwrap(),
            ),
            _ => (id.to_string(), expr.parse::<Expression>().unwrap()),
        })
        .collect_vec();

    // reduce it
    let mut old_len = 0;
    while map.len() != old_len {
        old_len = map.len();
        map = reduce(map);

        map = map
            .iter()
            .map(|(id, expr)| (id.clone(), expr.clone().calculate()))
            .collect();
    }

    // flip it for humn
    map = map
        .iter()
        .map(|(id, expr)| expr.clone().flip(id.clone()))
        .collect();

    // reduce again
    while map.len() > 1 {
        map = reduce(map);

        map = map
            .iter()
            .map(|(id, expr)| (id.clone(), expr.clone().calculate()))
            .collect();
    }

    if let Expression::Literal(Value::Number(x)) = map[0].1 {
        Some(x)
    } else {
        None
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
