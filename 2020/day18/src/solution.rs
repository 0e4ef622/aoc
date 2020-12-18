use std::collections::*;
use rand::random;
use serde_scan::scan as s;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Token {
    Num(i64),
    Add,
    Mul,
    Lp,
    Rp,
}

impl Token {
    fn num(&self) -> Option<i64> {
        match self {
            Token::Num(n) => Some(*n),
            _ => None,
        }
    }
}

fn eval(tokens: &[Token]) -> (i64, usize) {
    let mut i = 1;
    let mut num = match tokens[0] {
        Token::Num(n) => n,
        Token::Lp => {
            let r = eval(&tokens[1..]);
            i += r.1+1;
            r.0
        }
        _ => panic!("wtf"),
    };
    let mut op = None;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Add => op = Some(Token::Add),
            Token::Mul => op = Some(Token::Mul),
            Token::Num(n) => match op {
                Some(Token::Add) => {
                    num += n;
                    op = None;
                }
                Some(Token::Mul) => {
                    num *= n;
                    op = None;
                }
                _ => panic!("wtf {:?} {:?} {}", tokens, op, i),
            }
            Token::Lp => match op {
                Some(Token::Add) => {
                    let r = eval(&tokens[i+1..]);
                    num += r.0;
                    i += r.1+1;
                    op = None;
                }
                Some(Token::Mul) => {
                    let r = eval(&tokens[i+1..]);
                    num *= r.0;
                    i += r.1+1;
                    op = None;
                }
                _ => panic!("wtf"),
            }
            Token::Rp => return (num, i),
        }
        i += 1;
    }
    (num, i)
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    for line in input.lines() {
        let mut tokens = vec![];
        let mut n = 0;
        for ch in line.bytes() {
            match ch {
                b'0'..=b'9' => n = 10*n + (ch - b'0') as i64,
                b'+' => {
                    if n != 0 { tokens.push(Token::Num(n)); }
                    tokens.push(Token::Add);
                    n = 0;
                }
                b'*' => {
                    if n != 0 { tokens.push(Token::Num(n)); }
                    tokens.push(Token::Mul);
                    n = 0;
                }
                b'(' => {
                    if n != 0 { tokens.push(Token::Num(n)); }
                    tokens.push(Token::Lp);
                    n = 0;
                }
                b')' => {
                    if n != 0 { tokens.push(Token::Num(n)); }
                    tokens.push(Token::Rp);
                    n = 0;
                }
                _ => (),
            }
        }
        if n != 0 { tokens.push(Token::Num(n)); }
        sum += eval(&tokens).0;
    }
    sum
}

fn eval2(tokens: &[Token]) -> i64 {
    let mut output = vec![];
    let mut opstack = vec![];
    for token in dbg!(tokens) {
        match token {
            Token::Num(_) => output.push(*token),
            Token::Mul => { while opstack.last() == Some(&Token::Add) { output.push(opstack.pop().unwrap()); } opstack.push(*token); }
            Token::Add => { while opstack.last() == Some(&Token::Add) { output.push(opstack.pop().unwrap()); } opstack.push(*token); }
            Token::Lp => opstack.push(*token),
            Token::Rp => {
                while *opstack.last().unwrap() != Token::Lp {
                    match opstack.pop().unwrap() {
                        Token::Add => output.push(Token::Add),
                        Token::Mul => output.push(Token::Mul),
                        _ => unreachable!(),
                    }
                }
                opstack.pop();
            }
        }
    }
    opstack.reverse();
    output.extend(opstack);

    let mut stack = vec![];
    for thing in output {
        match thing {
            Token::Num(n) => stack.push(n),
            Token::Add => {
                let n = stack.pop().unwrap();
                *stack.last_mut().unwrap() += n;
            }
            Token::Mul => {
                let n = stack.pop().unwrap();
                *stack.last_mut().unwrap() *= n;
            }
            _ => unreachable!(),
        }
    }
    stack[0]
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    for line in input.lines() {
        let mut tokens = vec![];
        let mut n = 0;
        for ch in line.bytes() {
            match ch {
                b'0'..=b'9' => n = 10*n + (ch - b'0') as i64,
                b'+' => {
                    if n != 0 { tokens.push(Token::Num(n)); }
                    tokens.push(Token::Add);
                    n = 0;
                }
                b'*' => {
                    if n != 0 { tokens.push(Token::Num(n)); }
                    tokens.push(Token::Mul);
                    n = 0;
                }
                b'(' => {
                    if n != 0 { tokens.push(Token::Num(n)); }
                    tokens.push(Token::Lp);
                    n = 0;
                }
                b')' => {
                    if n != 0 { tokens.push(Token::Num(n)); }
                    tokens.push(Token::Rp);
                    n = 0;
                }
                _ => (),
            }
        }
        if n != 0 { tokens.push(Token::Num(n)); }
        sum += eval2(&tokens);
    }
    sum
}
