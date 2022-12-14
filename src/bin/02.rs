#[derive(PartialEq, Debug)]
enum RPSAction {
    Rock,
    Paper,
    Scissors,
}

impl From<i32> for RPSAction {
    fn from(value: i32) -> Self {
        match value % 3 {
            0 => RPSAction::Rock,
            1 => RPSAction::Paper,
            2 => RPSAction::Scissors,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<&str> for RPSAction {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(RPSAction::Rock),
            "B" | "Y" => Ok(RPSAction::Paper),
            "C" | "Z" => Ok(RPSAction::Scissors),
            _ => {
                println!("Error unknown string \"{:?}\"", value);
                Err(())
            }
        }
    }
}

#[derive(PartialEq, Debug)]
enum RPSResult {
    Loss,
    Draw,
    Win,
}

impl TryFrom<&str> for RPSResult {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(RPSResult::Loss),
            "Y" => Ok(RPSResult::Draw),
            "Z" => Ok(RPSResult::Win),
            _ => {
                println!("Error unknown string \"{:?}\"", value);
                Err(())
            }
        }
    }
}

impl RPSAction {
    fn value(&self) -> i32 {
        match self {
            RPSAction::Rock => 0,
            RPSAction::Paper => 1,
            RPSAction::Scissors => 2,
        }
    }

    fn result_against(&self, opponent: &RPSAction) -> RPSResult {
        match self.value() {
            v if v == opponent.value() => RPSResult::Draw,
            v if v == ((opponent.value() + 1).rem_euclid(3)) => RPSResult::Win,
            v if v == ((opponent.value() - 1).rem_euclid(3)) => RPSResult::Loss,
            _ => unreachable!(),
        }
    }

    fn score_against(&self, opponent: &RPSAction) -> i32 {
        match self.result_against(opponent) {
            RPSResult::Loss => self.value() + 1,
            RPSResult::Draw => self.value() + 4,
            RPSResult::Win => self.value() + 7,
        }
    }

    fn move_for_result(&self, result: &RPSResult) -> RPSAction {
        match result {
            RPSResult::Loss => RPSAction::from((self.value() - 1).rem_euclid(3)),
            RPSResult::Draw => RPSAction::from(self.value()),
            RPSResult::Win => RPSAction::from((self.value() + 1).rem_euclid(3)),
        }
    }

    fn score_for_result(&self, result: &RPSResult) -> i32 {
        let m = self.move_for_result(result);
        match result {
            RPSResult::Loss => m.value() + 1,
            RPSResult::Draw => m.value() + 4,
            RPSResult::Win => m.value() + 7,
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let it = input.lines().map(|l| {
        let (a, b) = l.split_at(1);
        (
            RPSAction::try_from(a.trim()).unwrap(),
            RPSAction::try_from(b.trim()).unwrap(),
        )
    });
    Some(it.map(|a| a.1.score_against(&a.0)).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let it = input.lines().map(|l| {
        let (a, b) = l.split_at(1);
        (
            RPSAction::try_from(a.trim()).unwrap(),
            RPSResult::try_from(b.trim()).unwrap(),
        )
    });
    Some(it.map(|a| a.0.score_for_result(&a.1)).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
