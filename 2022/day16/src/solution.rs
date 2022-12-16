use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct Valve {
    tunnels: Vec<(usize, u64)>,
    flow: i64,
}

fn floyd_warshall(valves: &[Valve]) -> Vec<Vec<u64>> {
    let mut d = vec![vec![99999999; valves.len()]; valves.len()];
    for i in 0..valves.len() {
        d[i][i] = 0;
        for &(ch, co) in &valves[i].tunnels {
            d[i][ch] = co;
        }
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
            }
        }
    }
    d
}

fn prune(g: &[Valve], d: &[Vec<u64>], idxs: &[usize]) -> Vec<Valve> {
    let mut gg: Vec<Valve> = vec![];
    for &i in idxs {
        gg.push(Valve {
            tunnels: vec![],
            flow: g[i].flow,
        });
        for (e, &j) in idxs.iter().enumerate() {
            if i == j { continue; }
            gg.last_mut().unwrap().tunnels.push((e, d[i][j]));
        }
    }
    gg
}

fn tsp(v: &[Valve], t: usize) -> i64 {
    let mut dp = vec![vec![vec![i64::MIN; 1<<v.len()]; v.len()]; t+1];
    dp[0][0][1] = 0;
    let mut ans = 0;
    for i in 0..t {
        for j in 0..v.len() {
            for b in 0..1<<v.len() {
                let cur = dp[i][j][b];
                for &(to, co) in &v[j].tunnels {
                    let co = co as usize;
                    if i + co + 1 > t { continue; }
                    if (b & (1 << to)) != 0 { continue; }
                    let nv = cur + (t - i - co - 1) as i64 * v[to].flow;
                    let ndp = &mut dp[i + co + 1][to][b | (1 << to)];
                    *ndp = nv.max(*ndp);
                    ans = ans.max(nv);
                }
            }
        }
    }
    ans
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut g: Vec<Valve> = vec![];
    let mut name_to_idx = HashMap::new();
    let mut get = |name: String, g: &mut Vec<Valve>| *name_to_idx.entry(name).or_insert_with(|| {
        g.push(Default::default());
        g.len() - 1
    });
    let mut nonzero_idx = vec![];
    for line in input.lines() {
        let parts = line.split([' ','=',';',',']).filter(|x| !x.is_empty()).cv();
        let name = parts[1].to_owned();
        let flow = parts[5].parse().unwrap();
        let tunnels = parts[10..].iter().map(|&t| (get(t.to_owned(), &mut g), 1)).cv();

        let i = get(name, &mut g);
        g[i] = Valve {
            tunnels,
            flow,
        };
        if flow != 0 {
            nonzero_idx.push(i);
        }
    }

    nonzero_idx.insert(0, name_to_idx["AA"]);
    let dist_matrix = floyd_warshall(&g);
    let pruned = prune(&g, &dist_matrix, &nonzero_idx);
    tsp(&pruned, 30)
}

fn tsp2(v: &[Valve], t: usize) -> i64 {
    let mut dp = vec![vec![vec![i64::MIN; 1<<v.len()]; v.len()]; t+1];
    dp[0][0][1] = 0;
    let mut ans = 0;
    for i in 0..t {
        for j in 0..v.len() {
            for b in 0..1<<v.len() {
                let cur = dp[i][j][b];
                dp[i+1][j][b] = dp[i+1][j][b].max(cur);
                for &(to, co) in &v[j].tunnels {
                    let co = co as usize;
                    if i + co + 1 > t { continue; }
                    if (b & (1 << to)) != 0 { continue; }
                    let nv = cur + (t - i - co - 1) as i64 * v[to].flow;
                    let ndp = &mut dp[i + co + 1][to][b | (1 << to)];
                    *ndp = nv.max(*ndp);
                    ans = ans.max(nv);
                }
            }
        }
    }

    let mut mx = 0;
    for b in 0..1<<v.len() {
        let mut m1 = 0;
        for i in 0..v.len() {
            m1 = m1.max(dp[t][i][b]);
        }
        let nb = b ^ ((1<<v.len())-2);
        let mut m2 = 0;
        let mut b = nb;
        loop {
            for i in 0..v.len() {
                m2 = m2.max(dp[t][i][b]);
            }
            if b == 0 { break; }
            b -= 1;
            b &= nb;
        }
        mx = mx.max(m1+m2);
    }
    mx
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut g: Vec<Valve> = vec![];
    let mut name_to_idx = HashMap::new();
    let mut get = |name: String, g: &mut Vec<Valve>| *name_to_idx.entry(name).or_insert_with(|| {
        g.push(Default::default());
        g.len() - 1
    });
    let mut nonzero_idx = vec![];
    for line in input.lines() {
        let parts = line.split([' ','=',';',',']).filter(|x| !x.is_empty()).cv();
        let name = parts[1].to_owned();
        let flow = parts[5].parse().unwrap();
        let tunnels = parts[10..].iter().map(|&t| (get(t.to_owned(), &mut g), 1)).cv();

        let i = get(name, &mut g);
        g[i] = Valve {
            tunnels,
            flow,
        };
        if flow != 0 {
            nonzero_idx.push(i);
        }
    }

    nonzero_idx.insert(0, name_to_idx["AA"]);
    let dist_matrix = floyd_warshall(&g);
    let pruned = prune(&g, &dist_matrix, &nonzero_idx);
    tsp2(&pruned, 26)
}
