const W: usize = 25;
const H: usize = 6;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.trim().chars().collect::<Vec<_>>();
    let l = input.chunks(W*H).map(|x| (x.iter().filter(|&&x| x=='0').count(), x)).min().unwrap();

    l.1.iter().filter(|&&x| x=='1').count() * l.1.iter().filter(|&&x| x=='2').count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.trim().chars().collect::<Vec<_>>();
    let mut a = [' '; W*H];
    for layer in input.chunks(W*H) {
        for (i, &ch) in layer.iter().enumerate() {
            if ch == '1' && a[i] == ' '{
                a[i] = '#';
            } else if ch == '0' && a[i] == ' '{
                a[i] = '.';
            }
        }
    }

    for ch in a.chunks(W) {
        for c in ch {
            print!("{}", c);
        }
        println!();
    }
    0
}
