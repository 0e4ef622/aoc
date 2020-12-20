use std::collections::*;
use rand::random;
use serde_scan::scan as s;

#[derive(Debug)]
struct Tile {
    id: usize,
    top: Vec<u8>,
    bottom: Vec<u8>,
    left: Vec<u8>,
    right: Vec<u8>,
    inner: Vec<Vec<u8>>,
    used: bool,
}

impl Tile {
    fn rotate(&mut self) {
        std::mem::swap(&mut self.top, &mut self.right);
        std::mem::swap(&mut self.top, &mut self.bottom);
        std::mem::swap(&mut self.top, &mut self.left);
        self.inner = rotate(&self.inner);
        self.bottom.reverse();
        self.top.reverse();
    }
    fn flipv(&mut self) {
        std::mem::swap(&mut self.top, &mut self.bottom);
        self.left.reverse();
        self.right.reverse();
        self.inner.reverse();
    }
}

fn mktile(tile: &str, id: usize) -> Tile {
    let lines = tile.lines().collect::<Vec<_>>();
    let top = lines[0].as_bytes().to_vec();
    let bottom = lines.last().unwrap().as_bytes().to_vec();
    let left = lines.iter().map(|x| x.as_bytes()[0]).collect::<Vec<_>>();
    let right = lines.iter().map(|x| *x.as_bytes().last().unwrap()).collect::<Vec<_>>();

    let inner = lines[1..lines.len()-1].iter().map(|x| x[1..x.len()-1].as_bytes().to_vec()).collect();
    Tile { top, bottom, left, right, id, used: false, inner }
}

fn attempt(tiles: &mut HashMap<usize, Tile>, grid: &mut [[Option<usize>; 12]; 12]) -> bool {
    let (x, y, _) = match grid.iter().enumerate().flat_map(|(y, r)| r.iter().enumerate().map(move |(x, e)| (x, y, e))).find(|x| *x.2 == None) {
        None => return true,
        Some(x) => x,
    };

    let tile_ids = tiles.keys().copied().collect::<Vec<_>>();

    for tile in tile_ids {
        if tiles[&tile].used { continue; }
        for _ in 0..4 {
            if x == 0 || tiles[&tile].left == tiles[&grid[y][x-1].unwrap()].right {
                if y == 0 || tiles[&tile].top == tiles[&grid[y-1][x].unwrap()].bottom {
                    tiles.get_mut(&tile).unwrap().used = true;
                    grid[y][x] = Some(tile);
                    if attempt(tiles, grid) { return true; }
                    tiles.get_mut(&tile).unwrap().used = false;
                    grid[y][x] = None;
                }
            }
            tiles.get_mut(&tile).unwrap().rotate();
        }
        tiles.get_mut(&tile).unwrap().flipv();
        for _ in 0..4 {
            if x == 0 || tiles[&tile].left == tiles[&grid[y][x-1].unwrap()].right {
                if y == 0 || tiles[&tile].top == tiles[&grid[y-1][x].unwrap()].bottom {
                    tiles.get_mut(&tile).unwrap().used = true;
                    grid[y][x] = Some(tile);
                    if attempt(tiles, grid) { return true; }
                    tiles.get_mut(&tile).unwrap().used = false;
                    grid[y][x] = None;
                }
            }
            tiles.get_mut(&tile).unwrap().rotate();
        }
    }
    false
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut tiles = HashMap::new();
    for section in input.split("\n\n") {
        let mut s = section.splitn(2, "\n");
        let tile_id = s.next().unwrap();
        let tile_id = tile_id[5..tile_id.len()-1].parse::<usize>().unwrap();
        let tile = s.next().unwrap();
        let tile = mktile(tile, tile_id);
        tiles.insert(tile_id, tile);
    }
    let mut grid = [[None; 12]; 12];
    let r = attempt(&mut tiles, &mut grid);
    assert!(r);
    grid[0][0].unwrap() * grid[11][0].unwrap() * grid[0][11].unwrap() * grid[11][11].unwrap()
}

fn rotate(pattern: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut new = vec![vec![0; pattern.len()]; pattern[0].len()];
    for y in 0..pattern.len() {
        for x in 0..pattern[0].len() {
            new[x][pattern.len()-y-1] = pattern[y][x];
        }
    }
    new
}

fn flip(pattern: &mut [Vec<u8>]) {
    pattern.reverse();
}

fn find(grid: &mut [Vec<u8>], pattern: &[Vec<u8>]) -> usize {
    let pw = pattern[0].len();
    let ph = pattern.len();

    let mut r = 0;

    for y in 0..=grid.len()-ph {
        'a: for x in 0..=grid[0].len()-pw {
            let mut c = 0;
            for yy in 0..ph {
                for xx in 0..pw {
                    if pattern[yy][xx] == b'#' && grid[y+yy][x+xx] != b'#' {
                        continue 'a;
                    } else if pattern[yy][xx] == b'#' && grid[y+yy][x+xx] == b'#' {
                        c += 1;
                    }
                }
            }
            r += c;
        }
    }
    r
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut tiles = HashMap::new();
    for section in input.split("\n\n") {
        let mut s = section.splitn(2, "\n");
        let tile_id = s.next().unwrap();
        let tile_id = tile_id[5..tile_id.len()-1].parse::<usize>().unwrap();
        let tile = s.next().unwrap();
        let tile = mktile(tile, tile_id);
        tiles.insert(tile_id, tile);
    }
    let mut grid = [[None; 12]; 12];
    let r = attempt(&mut tiles, &mut grid);
    assert!(r);
    let mut newgrid = vec![];
    for row in &grid {
        for line in 0..tiles[&row[0].unwrap()].inner.len() {
            let mut ngcol: Vec<u8> = vec![];
            for col in 0..12 {
                ngcol.extend(&tiles[&row[col].unwrap()].inner[line]);
            }
            newgrid.push(ngcol);
        }
    }

    let mut pattern = vec![
    b"                  # ".to_vec(),
    b"#    ##    ##    ###".to_vec(),
    b" #  #  #  #  #  #   ".to_vec()];


    let mut t = 0;
    for _ in 0..4 {
        t += find(&mut newgrid, &pattern);
        pattern = rotate(&pattern);
    }

    flip(&mut pattern);
    for _ in 0..4 {
        t += find(&mut newgrid, &pattern);
        pattern = rotate(&pattern);
    }
    newgrid.iter().flatten().filter(|&&x| x == b'#').count() - t
}
