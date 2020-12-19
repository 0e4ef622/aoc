use std::collections::*;
use rand::random;
use serde_scan::scan as s;


fn test_rule(rules: &HashMap<usize, Vec<Vec<Result<usize, u8>>>>, s: &mut &str, r: usize) -> bool {
    if s.is_empty() { return false; }
    'outer: for rule_seq in &rules[&r] {
        let mut ss = *s;
        for rule in rule_seq {
            if ss.is_empty() { continue 'outer; }
            match rule {
                Err(c) => {
                    if ss.as_bytes()[0] != *c {
                        continue 'outer;
                    } else {
                        ss = &ss[1..];
                    }
                }
                Ok(r) => {
                    if !test_rule(rules, &mut ss, *r) {
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
            .map(|x| x.trim().split_whitespace().map(|x| x.parse().map_err(|_| x.as_bytes()[1])).collect::<Vec<_>>())
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
            .map(|x| x.trim().split_whitespace().map(|x| x.parse().map_err(|_| x.as_bytes()[1])).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        rules.insert(r.parse().unwrap(), a);
    }

    sections[1].lines()
        .map(|mut line| {
            let mut cnt42 = 0;
            loop {
                if !test_rule(&rules, &mut line, 42) { break; }
                cnt42 += 1;

                let mut line = line;
                let mut cnt31 = 0;
                while test_rule(&rules, &mut line, 31) { cnt31 += 1; }
                if cnt42 > cnt31 && cnt42 >= 2 && cnt31 >= 1 && line.is_empty() { return true; }
            }
            return false;
        })
        .filter(|x| *x)
        .count()
}
