use crate::utils::*;

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(i64),
    Addition,
    Multiply,
    Open,
    Close,
}

impl Token {
    fn from(c: char) -> Token {
        match c {
            '+' => Token::Addition,
            '*' => Token::Multiply,
            '(' => Token::Open,
            ')' => Token::Close,
            '0'..='9' => Token::Number(c.to_digit(10).unwrap() as i64),
            _ => panic!("Invalid char: {}", c),
        }
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");
    // let input = "1 + 2 * 3 + 4 * 5 + 6";

    let mut parsed = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| *c != ' ')
                .map(Token::from)
                .rev()
                .to_vec()
        })
        .to_vec();

    fn next_value(tokens: &mut Vec<Token>) -> i64 {
        match tokens.pop().unwrap() {
            Token::Open => evaluate(tokens),
            Token::Number(n) => n,
            t => panic!("Expected Number or '(', got {:?}", t),
        }
    }

    fn evaluate(tokens: &mut Vec<Token>) -> i64 {
        let mut value = next_value(tokens);
        let mut other_value: Option<i64> = None;

        while !tokens.is_empty() {
            match tokens.pop().unwrap() {
                Token::Close => {
                    if let Some(o) = other_value {
                        // println!("{} * {} = {}", o, value, value * o);
                        value *= o;
                    }
                    return value;
                }
                Token::Addition => {
                    let next = next_value(tokens);
                    if let Some(o) = other_value.as_mut() {
                        // println!("{} + {} = {}", *o, next, *o + next);
                        *o += next;
                    } else {
                        // println!("{} + {} = {}", value, next, value + next);
                        value += next
                    }
                }
                Token::Multiply => {
                    if let Some(o) = other_value {
                        // println!("{} * {} = {}", o, value, value * o);
                        value *= o;
                    }
                    other_value = Some(next_value(tokens));
                }
                t => panic!("Expected Operator, got {:?}", t),
            }
        }

        if let Some(o) = other_value {
            // println!("{} * {} = {}", o, value, value * o);
            value *= o;
        }
        value
    }

    let sum = parsed.iter_mut().map(evaluate).sum::<i64>();
    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");
    // let input = ;

    let mut parsed = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| *c != ' ')
                .map(Token::from)
                .rev()
                .to_vec()
        })
        .to_vec();

    fn next_value(tokens: &mut Vec<Token>) -> i64 {
        match tokens.pop().unwrap() {
            Token::Open => evaluate(tokens),
            Token::Number(n) => n,
            t => panic!("Expected Number or '(', got {:?}", t),
        }
    }

    fn evaluate(tokens: &mut Vec<Token>) -> i64 {
        let mut value = next_value(tokens);

        while !tokens.is_empty() {
            match tokens.pop().unwrap() {
                Token::Close => return value,
                Token::Addition => value += next_value(tokens),
                Token::Multiply => value *= next_value(tokens),
                t => panic!("Expected Operator, got {:?}", t),
            }
        }

        value
    }

    let sum = parsed.iter_mut().map(evaluate).sum::<i64>();
    pv!(sum);
}
