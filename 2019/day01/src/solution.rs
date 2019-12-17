
pub fn part1(input: &str) -> String {
    let mut inlines = input.lines();

    let mut sum = 0;

    while let Some(l) = inlines.next() {
        sum += l.parse::<usize>().unwrap() / 3 - 2;
    }
    format!("{}", sum)
}

pub fn part2(input: &str) -> String {
    let mut inlines = input.lines();

    let mut sum = 0;

    while let Some(l) = inlines.next() {

        let mut fuel = l.parse::<usize>().unwrap();

        while (fuel / 3).saturating_sub(2) > 0 {
            sum += (fuel / 3).saturating_sub(2);
            fuel = (fuel / 3).saturating_sub(2);
        }
    }

    let mut fuel = sum;

    format!("{}", sum)
}
