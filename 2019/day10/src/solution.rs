pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut map = input.trim().lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<_>>();

    let mut max = 0;
    let mut X = 0;
    let mut Y = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch == b'#' {
                let v = visible(&map, x as isize, y as isize).len();
                if v > max {
                    max = v;
                    X = x;
                    Y = y;
                }
            }
        }
    }

    max
}

fn visible(map: &[Vec<u8>], x: isize, y: isize) -> Vec<(isize, isize)> {

    let mut asts = vec![];

    for (_Y, line) in map.iter().enumerate() {
        'a: for (_X, &ch) in line.iter().enumerate() {
            let mut X = _X as isize;
            let mut Y = _Y as isize;
            if Y == y && X == x { continue; }
            if ch == b'#' {

                let mut dx = X - x;
                let mut dy = Y - y;
                let mut d = gcd(dx.abs(), dy.abs());

                dx /= d;
                dy /= d;
                X -= dx;
                Y -= dy;

                while X != x || Y != y {

                    if map[Y as usize][X as usize] == b'#' {
                        continue 'a;
                    }
                    X -= dx;
                    Y -= dy;
                }

                asts.push((_X as isize, _Y as isize));
            }
        }
    }
    asts
}

fn gcd(n: isize, m: isize) -> isize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

#[test]
fn test() {
    let map = [
        &b"#.."[..],
        &b".#."[..],
        &b"..#"[..],
    ];
    assert_eq!(visible(&map[..], 0, 0).len(), 2);

    let map = [
        &b"#.."[..],
        &b"..."[..],
        &b".#."[..],
        &b"..."[..],
        &b"..#"[..],
    ];
    assert_eq!(visible(&map[..], 2, 4).len(), 2);
    let map = [
        &b"..."[..],
        &b"..#"[..],
        &b"..#"[..],
        &b"..#"[..],
        &b"..#"[..],
    ];
    assert_eq!(visible(&map[..], 2, 1).len(), 2);
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut map = input.trim().lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<_>>();

    let mut max = 0;
    let mut X = 0;
    let mut Y = 0;
    let mut asts = vec![];
    for (y, line) in map.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch == b'#' {
                let v = visible(&map, x as isize, y as isize);
                if v.len() > max {
                    max = v.len();
                    X = x as isize;
                    Y = y as isize;
                    asts = v;
                }
            }
        }
    }

    asts.sort_unstable_by(|a, b| {
        let n = ((((a.1 - Y) as f64).atan2((a.0 - X) as f64) + std::f64::consts::PI/2.0)).rem_euclid( 2.0 * std::f64::consts::PI);
        let m = ((((b.1 - Y) as f64).atan2((b.0 - X) as f64) + std::f64::consts::PI/2.0)).rem_euclid( 2.0*std::f64::consts::PI);
        n.partial_cmp(&m).unwrap()
    });

    asts.reverse();
    let mut vapc = 0;
    loop {
        if asts.is_empty() {
            asts = visible(&map, X as isize, Y as isize);
            // asts.sort_unstable_by_key(|(x, y)| {
            //     (y as f64).atan2(x as f64) + std::f64::consts::PI/4.0
            // });
            asts.sort_unstable_by(|a, b| {
        let n = ((((a.1 - Y) as f64).atan2((a.0 - X) as f64) + std::f64::consts::PI/2.0)).rem_euclid( 2.0 * std::f64::consts::PI);
        let m = ((((b.1 - Y) as f64).atan2((b.0 - X) as f64) + std::f64::consts::PI/2.0)).rem_euclid( 2.0*std::f64::consts::PI);
                n.partial_cmp(&m).unwrap()
            });
    asts.reverse();
        }

        let ast = asts.pop().unwrap();
        // println!("zapping {} {}", ast.0, ast.1);
        vapc += 1;
        map[ast.1 as usize][ast.0 as usize] = b'.';
        if vapc == 200 {
            return format!("{} {}", ast.0, ast.1);
        }
    }
}
