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
