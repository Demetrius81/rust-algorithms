#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &[Vec<(usize, usize)>], start: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; graph.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }

        for &(next, weight) in &graph[position] {
            let next_cost = cost + weight;

            if next_cost < dist[next] {
                heap.push(State { cost: next_cost, position: next });
                dist[next] = next_cost;
            }
        }
    }

    dist
}

fn dijkstra_with_path(graph: &[Vec<(usize, usize)>], start: usize) -> (Vec<usize>, Vec<Option<usize>>) {
    let mut dist = vec![usize::MAX; graph.len()];
    let mut prev = vec![None; graph.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }

        for &(next, weight) in &graph[position] {
            let next_cost = cost + weight;
            if next_cost < dist[next] {
                heap.push(State { cost: next_cost, position: next });
                dist[next] = next_cost;
                prev[next] = Some(position);
            }
        }
    }

    (dist, prev)
}

fn get_path(prev: &[Option<usize>], mut end: usize) -> Vec<usize> {
    let mut path = vec![];
    while let Some(p) = prev[end] {
        path.push(end);
        end = p;
    }
    path.push(end);
    path.reverse();
    path
}

pub fn run() {
    let graph = vec![
        vec![(1, 2), (2, 4)],  // Vertex 0 is connected to 1 (weight 2) and 2 (weight 4)
        vec![(2, 1), (3, 7)],  // Vertex 0 is connected to 2 (weight 1) and 3 (weight 7)
        vec![(3, 3)],          // Vertex 2 is connected to 3 (weight 3)
        vec![],                // Vertex 3 is not connected to anything
    ];

    let dist = dijkstra(&graph, 0);
    println!("{:?}", dist);

    let graph = vec![
        vec![(1, 2), (2, 4)],
        vec![(2, 1), (3, 7)],
        vec![(3, 3)],
        vec![],
    ];

    let (dist, prev) = dijkstra_with_path(&graph, 0);
    println!("Shortest distances: {:?}", dist);
    println!("The way to the 3rd peak: {:?}", get_path(&prev, 3));
}
