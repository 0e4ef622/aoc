use std::collections::*;

trait ApplyTo: Sized {
    fn app<F, T>(self, f: F) -> T
    where
        F: FnOnce(Self) -> T,
    {
        f(self)
    }
}

impl<T> ApplyTo for T {}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut food = vec![];
    let mut ingg = HashSet::new();
    let mut maybe = HashMap::new();
    for line in input.lines() {
        let mut s = line.split('(');
        let ing = s.next().unwrap().trim().split_whitespace().inspect(|&x| { ingg.insert(x); }).collect::<HashSet<_>>();
        let alleg = s.next().unwrap().app(|x| &x[9..x.len()-1]).split(", ").collect::<HashSet<_>>();
        food.push((ing, alleg));
    }

    for f in &food {
        for allergy in &f.1 {
            maybe.entry(allergy).or_insert(f.0.clone())
                .app(|x| *x = &*x & &f.0);
        }
    }

    for e in maybe.values() {
        ingg = &ingg - e;
    }

    food.iter()
        .flat_map(|x| &x.0)
        .filter(|x| ingg.contains(*x))
        .count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut food = vec![];
    let mut ingg = HashSet::new();
    let mut maybe = HashMap::new();
    for line in input.lines() {
        let mut s = line.split('(');
        let ing = s.next().unwrap().trim().split_whitespace().inspect(|&x| { ingg.insert(x); }).collect::<HashSet<_>>();
        let alleg = s.next().unwrap().app(|x| &x[9..x.len()-1]).split(", ").collect::<HashSet<_>>();
        food.push((ing, alleg));
    }

    for f in &food {
        for &allergy in &f.1 {
            maybe.entry(allergy).or_insert(f.0.clone())
                .app(|x| *x = &*x & &f.0);
        }
    }

    for e in maybe.values() {
        ingg = &ingg - e;
    }

    let mut ingaleg = vec![];
    while let Some(thing) = maybe.iter().find(|x| x.1.len() == 1).map(|x| *x.0) {
        let ing = *maybe[thing].iter().next().unwrap();
        ingaleg.push((thing, ing));
        maybe.remove(thing);
        maybe.values_mut().for_each(|v| { v.remove(ing); });
    }

    ingaleg.sort_unstable_by_key(|x| x.0);
    let mut s = String::new();
    ingaleg.iter().for_each(|x| { s += x.1; s += ","; });
    s.pop();
    s
}
