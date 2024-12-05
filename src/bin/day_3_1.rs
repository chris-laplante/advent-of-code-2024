use muncher::Muncher;

enum ParserState {
    ExpectMul,
    EatOperand1,
    EatOperand2 {
        operand1: u64,
    },
}

trait MuncherAdvance {
    fn advance(&mut self, count: usize);
}

impl MuncherAdvance for Muncher<'_> {
    fn advance(&mut self, count: usize) {
        // TODO: this is dumb, just add method to Muncher directly and set 'next'
        for _ in 0..count {
            self.eat();
        }
    }
}

fn calculate(input: &str) -> u64 {
    let mut m = Muncher::new(input);
    let mut state = ParserState::ExpectMul;

    let mut operations: Vec<(u64, u64)> = vec![];

    while !m.is_done() {
        match state {
            ParserState::ExpectMul => {
                m.eat_range_of("mul(");
                m.advance(4);

                state = ParserState::EatOperand1;
            }
            ParserState::EatOperand1 => {
                let d = m.eat_range_of(",");
                let maybe_operand1 = &m.text()[d.0..d.1];
                if let Ok(operand1) = maybe_operand1.parse::<u64>() {
                    if m.eat_comma() {
                        state = ParserState::EatOperand2 { operand1 };
                        continue;
                    }
                }

                m.advance(d.1 - d.0);
                state = ParserState::ExpectMul;
            }
            ParserState::EatOperand2 { operand1 } => {
                let d = m.eat_range_of(")");
                let maybe_operand2 = &m.text()[d.0..d.1];
                if let Ok(operand2) = maybe_operand2.parse::<u64>() {
                    operations.push((operand1, operand2));
                    assert!(m.eat_close_paren());
                }

                state = ParserState::ExpectMul;
            }
        }
    }

    dbg!(&operations);

    let result = operations.into_iter().fold(0u64, |acc, op| acc + (op.0 * op.1));
    dbg!(result);
    result
}

fn main() {
    let input = include_str!("../../resources/day_3/puzzle_3.txt");

    calculate(input);
}

#[cfg(test)]
mod test {
    use crate::calculate;

    #[test]
    fn simple1() {
        let input = "mul(11,8)mul(8,5)";
        assert_eq!(calculate(input), 10);
    }
}