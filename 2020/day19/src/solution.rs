use std::collections::*;
use rand::random;
use serde_scan::scan as s;


fn oldtest_rule(rules: &HashMap<usize, Vec<Vec<&str>>>, s: &mut &str, r: usize) -> bool {
    println!("test({}, {})", s, r);
    if s.is_empty() { return false; }
    'outer: for rule_seq in &rules[&r] {
        let mut ss = *s;
        for rule in rule_seq {
            if ss.is_empty() { continue 'outer; }
            match rule.as_bytes()[0] {
                b'\"' => {
                    if ss.as_bytes()[0] != rule.as_bytes()[1] {
                        continue 'outer;
                    } else {
                        ss = &ss[1..];
                    }
                }
                _ => {
                    if !test_rule(rules, &mut ss, rule.parse().unwrap()) {
                        continue 'outer;
                    }
                }
            }
        }
        *s = ss;
        return true;
    }
    false
}

fn test_rule(rules: &HashMap<usize, Vec<Vec<&str>>>, s: &mut &str, r: usize) -> bool {
    if s.is_empty() { return false; }
    'outer: for rule_seq in &rules[&r] {
        let mut ss = *s;
        for rule in rule_seq {
            if ss.is_empty() { continue 'outer; }
            match rule.as_bytes()[0] {
                b'\"' => {
                    if ss.as_bytes()[0] != rule.as_bytes()[1] {
                        continue 'outer;
                    } else {
                        ss = &ss[1..];
                    }
                }
                _ => {
                    if !test_rule(rules, &mut ss, rule.parse().unwrap()) {
                        continue 'outer;
                    }
                }
            }
        }
        *s = ss;
        return true;
    }
    false
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let sections: Vec<_> = input.split("\n\n").collect();
    let mut rules = HashMap::new();
    for rule in sections[0].lines() {
        let mut s = rule.split(": ");
        let r = s.next().unwrap();
        let a = s.next().unwrap()
            .split("|")
            .map(|x| x.trim().split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        rules.insert(r.parse().unwrap(), a);
    }

    sections[1].lines()
        .map(|mut line| test_rule(&rules, &mut line, 0) && line.is_empty())
        .filter(|x| *x)
        .count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let sections: Vec<_> = input.split("\n\n").collect();
    let mut rules = HashMap::new();
    for rule in sections[0].lines() {
        let mut s = rule.split(": ");
        let r = s.next().unwrap();
        let a = s.next().unwrap()
            .split("|")
            .map(|x| x.trim().split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        rules.insert(r.parse().unwrap(), a);
    }

    let rule11 = (rules[&11][0][0], rules[&11][0][1]);
    rules.get_mut(&11).unwrap().insert(0, vec![rule11.0, "11", rule11.1]);

    sections[1].lines()
        .map(|line| {
            for cnt8 in 1..101 {
                let mut line = line;
                (0..cnt8).for_each(|_| { test_rule(&rules, &mut line, 8); });
                if line.is_empty() { break; }
                if test_rule(&rules, &mut line, 11) && line.is_empty() {
                    return true;
                }
            }
            return false;
        })
        .filter(|x| *x)
        .count()
}
