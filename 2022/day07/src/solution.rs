use std::collections::*;
use rand::random;
use itertools::{iproduct, Itertools};
use util::*;

#[derive(Default, Clone)]
struct Node {
    size: usize,
    children: HashMap<String, usize>,
    parent: usize,
}

fn dfs(i: usize, g: &mut [Node]) {
    if g[i].children.is_empty() {
        return;
    }

    for ch in g[i].children.values().copied().cv() {
        dfs(ch, g);
        g[i].size += g[ch].size;
    }
}

fn print_tree(i: usize, prefix: &mut String, name: &str, g: &mut [Node]) {
    // .
    // ├── benches
    // │   └── bench.rs
    // ├── Cargo.toml
    // ├── exin
    // ├── in
    // └── src
    //     ├── lib.rs
    //     ├── main.rs
    //     └── solution.rs
    println!("{name} ({})", g[i].size);

    let ch = g[i].children.clone().into_iter().cv();
    for (j, (k, c)) in ch.iter().enumerate() {

        if j == ch.len() - 1 {
            print!("{}└── ", prefix);
            prefix.push_str("    ");
        } else {
            print!("{}├── ", prefix);
            prefix.push_str("│   ");
        }
        print_tree(*c, prefix, &k, g);

        for i in 0..4 {
            prefix.pop();
        }
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut tree: Vec<Node> = Default::default();
    tree.insert(0, Default::default());
    let mut cur_node: usize = 0;
    for sec in input.split("$ ").skip(2) {
        let lines = sec.lines().cv();
        let cmdwords = lines[0].split_whitespace().cv();
        if cmdwords[0] == "cd" {
            if cmdwords[1] == ".." {
                cur_node = tree[cur_node].parent.clone();
            } else {
                cur_node = tree[cur_node].children[cmdwords[1]].clone();
            }
        } else {
            for entry in &lines[1..] {
                let words = entry.split_whitespace().cv();
                let id = tree.len();
                tree[cur_node].children.insert(words[1].into(), id);
                if words[0] == "dir" {
                    tree.insert(id, Node { parent: cur_node.clone(), ..Default::default() });
                } else {
                    tree.insert(id, Node {
                        parent: cur_node.clone(),
                        size: words[0].parse::<usize>().unwrap(),
                        ..Default::default()
                    });
                }
            }
        }
    }
    dfs(0, &mut tree);
    // println!();
    // print_tree(0, &mut String::new(), "/", &mut tree);
    // println!();

    let mut ans = 0;
    for (i, v) in tree.iter().enumerate() {
        if v.size <= 100000 && !v.children.is_empty() {
            ans += v.size;
        }
    }
    ans
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut tree: Vec<Node> = Default::default();
    tree.insert(0, Default::default());
    let mut cur_node: usize = 0;
    for sec in input.split("$ ").skip(2) {
        let lines = sec.lines().cv();
        let cmdwords = lines[0].split_whitespace().cv();
        if cmdwords[0] == "cd" {
            if cmdwords[1] == ".." {
                cur_node = tree[cur_node].parent.clone();
            } else {
                cur_node = tree[cur_node].children[cmdwords[1]].clone();
            }
        } else {
            for entry in &lines[1..] {
                let words = entry.split_whitespace().cv();
                let id = tree.len();
                tree[cur_node].children.insert(words[1].into(), id);
                if words[0] == "dir" {
                    tree.insert(id, Node { parent: cur_node.clone(), ..Default::default() });
                } else {
                    tree.insert(id, Node {
                        parent: cur_node.clone(),
                        size: words[0].parse::<usize>().unwrap(),
                        ..Default::default()
                    });
                }
            }
        }
    }
    dfs(0, &mut tree);

    let mut ans = usize::MAX;
    let mut free = 70000000 - tree[0].size;
    for (i, v) in tree.iter().enumerate() {
        if v.size >= 30000000 - free && !v.children.is_empty() {
            ans = ans.min(v.size);
        }
    }
    ans
}
