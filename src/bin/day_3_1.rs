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
                let maybe_operand1 = m.peek_until_count(|ch| ch == &',');
                let maybe_operand1 = &m.text()[maybe_operand1.0..maybe_operand1.1];

                if let Ok(operand1) = maybe_operand1.parse::<u64>() {
                    if m.peek() == Some(&',') {
                        m.advance(maybe_operand1.len() + 1);
                        state = ParserState::EatOperand2 { operand1 };
                        continue;
                    }
                }

                dbg!(maybe_operand1);
                state = ParserState::ExpectMul;
            }
            ParserState::EatOperand2 { operand1 } => {
                let maybe_operand2 = m.peek_until_count(|ch| ch == &')');
                let maybe_operand2 = &m.text()[maybe_operand2.0..maybe_operand2.1];

                if let Ok(operand2) = maybe_operand2.parse::<u64>() {
                    operations.push((operand1, operand2));
                    m.advance(maybe_operand2.len() + 1);
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
    fn zero() {
        assert_eq!(calculate(""), 0);
    }

    #[test]
    fn simple0() {
        assert_eq!(calculate("mul(1,2)"), 2);
    }

    #[test]
    fn simple1() {
        let input = "mul(11,8)mul(8,5)";
        assert_eq!(calculate(input), 128);
    }

    #[test]
    fn longer0() {
        let input = "' mul(382,128)select(){*who(710,947)mul(117,325)?$#from()/select()mul(829,251)}@mul(17,183)(:*when()}?+,what()mul(911,142)";
        assert_eq!(calculate(input), (382*128)+(117*325)+(829*251)+(17*183)+(911*142));
    }

    #[test]
    fn longer1() {
        let input = "' mul(382,128)select(){*who(710,947)mul(117325)?$#from()/select()mul(829,251)}@mul(17,183)(:*when()}?+,what()mul(911,142)";
        assert_eq!(calculate(input), (382*128)+(829*251)+(17*183)+(911*142));
    }
}