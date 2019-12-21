use std::{cmp::Reverse, collections::{HashSet, BinaryHeap}};
use std::hash::Hash;

/// Returns the length of the shortest path from `initial` to `target` using a `HashSet` to keep
/// track of visited nodes. `neighbors` should push all nodes connected to its first parameter
/// along with the edge weight to the `Vec` in its second parameter.
pub fn dijkstra<N, F>(initial: N, target: N, neighbors: F) -> Option<usize>
where
    N: Clone + Hash + Eq,
    F: FnMut(N, &mut Vec<(usize, N)>),
{
    dijkstra_with_visited_contains(initial, target, neighbors, HashSet::new(), |v, n| v.insert(n), |v, n| v.contains(&n))
}

/// Returns the length of the shortest path from `initial` to `target` using a custom function to
/// keep track of visited nodes. `neighbors` should push all nodes connected to its first parameter
/// along with the edge weight to the `Vec` in its second parameter. `set_insert` should add its
/// only parameter to the set of visited nodes and return true if the node has not already been
/// visited, and false otherwise (this is meant to mirror `HashSet::insert`.
pub fn dijkstra_with_visited<N, F, G>(
    initial: N,
    target: N,
    neighbors: F,
    mut set_insert: G,
) -> Option<usize>
where
    N: Clone + Eq,
    F: FnMut(N, &mut Vec<(usize, N)>),
    G: FnMut(N) -> bool,
{
    dijkstra_with_visited_contains(initial, target, neighbors, (), |_, n| set_insert(n), |_, _| false)
}

/// Returns the length of the shortest path from `initial` to `target` using a custom function to
/// keep track of visited nodes. `neighbors` should push all nodes connected to its first parameter
/// along with the edge weight to the `Vec` in its second parameter. `set_insert` should add its
/// only parameter to the set of visited nodes and return true if the node has not already been
/// visited, and false otherwise (this is meant to mirror `HashSet::insert`. `set_contains` should
/// check if its only parameter has already been visited.
pub fn dijkstra_with_visited_contains<N, V, F, G, H>(
    initial: N,
    target: N,
    mut neighbors: F,
    mut visited_set: V,
    mut set_insert: G,
    mut set_contains: H,
) -> Option<usize>
where
    N: Clone + Eq,
    F: FnMut(N, &mut Vec<(usize, N)>),
    G: FnMut(&mut V, N) -> bool,
    H: FnMut(&V, N) -> bool,
{
    let mut heap = BinaryHeap::new();
    heap.push(Attached(Reverse(0), initial));

    let mut neighbor_vec = vec![];

    loop {

        let Attached(Reverse(total_dist), node) = heap.pop()?;

        if node == target {
            break Some(total_dist);
        }

        if !set_insert(&mut visited_set, node.clone()) { continue; }

        neighbors(node, &mut neighbor_vec);
        for (dist, neighbor) in neighbor_vec.drain(..) {
            if !set_contains(&visited_set, neighbor.clone()) {
                heap.push(Attached(Reverse(total_dist + dist), neighbor));
            }
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Attached<T, U>(T, U);
impl<T: PartialEq, U> PartialEq for Attached<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
    fn ne(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }
}

impl<T: Eq, U> Eq for Attached<T, U> {}

impl<T: PartialOrd, U> PartialOrd for Attached<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }
    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }
    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }
    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }
}

impl<T: Ord, U> Ord for Attached<T, U> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
