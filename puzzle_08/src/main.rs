use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 08!");

    let lines = read_to_string("input.txt").unwrap();
    let input = parse_input(&*lines);

    let adj = add_n_shortest_edges(input, 1000);
    let circuits = circuits_after_n_edges(adj);

    let result = circuits[..3].iter().map(|n| *n as u32).reduce(|a, b| a*b).unwrap();

    println!("Result: {result}")
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(l: &str) -> Vec<usize> {
    l.split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn add_n_shortest_edges(input: Vec<Vec<usize>>, n: usize) -> Vec<HashSet<usize>> {
    let mut weighted_edges: Vec<(usize, usize, f64)> = vec![];
    for (i, p_i) in input.iter().enumerate() {
        for (j, p_j) in input.iter().enumerate() {
            if i > j {
                let diff1 = (p_i[0] as f64) - (p_j[0] as f64);
                let diff2 = (p_i[1] as f64) - (p_j[1] as f64);
                let diff3 = (p_i[2] as f64) - (p_j[2] as f64);
                let square_diffs = diff1.powf(2.0) + diff2.powf(2.0) + diff3.powf(2.0);
                weighted_edges.push((i, j, square_diffs.sqrt()))
            }
        }
    }
    weighted_edges.sort_by(|(_, _, w1), (_, _, w2)| w1.partial_cmp(w2).unwrap());

    let mut adj: Vec<HashSet<usize>> = vec![];
    for _ in 0..input.len() {
        adj.push(HashSet::new());
    }
    for k in 0..n {
        let (i, j, _) = weighted_edges[k as usize];
        adj[i].insert(j);
        adj[j].insert(i);
    }

    adj
}

fn circuits_after_n_edges(adj: Vec<HashSet<usize>>) -> Vec<usize> {
    let mut circuits: Vec<usize> = vec![];
    let mut visited: Vec<bool> = vec![];
    for _ in 0..adj.len() {
        visited.push(false);
    }

    for u in 0..adj.len() {
        if !visited[u] {
            let circuit = bfs(&adj, u, &mut visited);
            circuits.push(circuit);
        }
    }

    circuits.sort();
    circuits.reverse();

    circuits
}

fn bfs(adj: &Vec<HashSet<usize>>, u: usize, visited: &mut Vec<bool>) -> usize {
    let mut circuit: Vec<usize> = vec![];

    let mut q: VecDeque<usize> = VecDeque::new();
    visited[u] = true;
    q.push_back(u);

    while let Some(v) = q.pop_front() {
        circuit.push(v);

        for w in &adj[v] {
            if !visited[*w] {
                visited[*w] = true;
                q.push_back(*w);
            }
        }
    }

    circuit.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "162,817,812\n\
                           57,618,57\n\
                           906,360,560\n\
                           542,29,236\n\
                           431,825,988\n\
                           52,470,668\n\
                           216,146,977\n\
                           805,96,715\n\
                           941,993,340\n\
                           862,61,35\n\
                           984,92,344\n\
                           425,690,689\n";

        assert_eq!(parse_input(input), [
            [162,817,812],
            [57,618,57],
            [906,360,560],
            [542,29,236],
            [431,825,988],
            [52,470,668],
            [216,146,977],
            [805,96,715],
            [941,993,340],
            [862,61,35],
            [984,92,344],
            [425,690,689]
        ])
    }

    #[test]
    fn test_add_n_shortest_edges() {
        let input = "162,817,812\n\
                           57,618,57\n\
                           906,360,560\n\
                           542,29,236\n\
                           431,825,988\n\
                           52,470,668\n\
                           216,146,977\n\
                           805,96,715\n\
                           941,993,340\n\
                           862,61,35\n\
                           984,92,344\n\
                           425,690,689\n";

        assert_eq!(add_n_shortest_edges(parse_input(input), 4), [
            HashSet::from([4, 11]),
            HashSet::new(),
            HashSet::from([7]),
            HashSet::new(),
            HashSet::from([0, 11]),
            HashSet::new(),
            HashSet::new(),
            HashSet::from([2]),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::from([0, 4]),
        ])
    }

    #[test]
    fn test_circuits_after_n_edges() {
        let input = "162,817,812\n\
                           57,618,57\n\
                           906,360,560\n\
                           592,479,940\n\
                           352,342,300\n\
                           466,668,158\n\
                           542,29,236\n\
                           431,825,988\n\
                           739,650,466\n\
                           52,470,668\n\
                           216,146,977\n\
                           819,987,18\n\
                           117,168,530\n\
                           805,96,715\n\
                           346,949,466\n\
                           970,615,88\n\
                           941,993,340\n\
                           862,61,35\n\
                           984,92,344\n\
                           425,690,689\n";

        assert_eq!(circuits_after_n_edges(add_n_shortest_edges(parse_input(input), 1)), [2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(circuits_after_n_edges(add_n_shortest_edges(parse_input(input), 2)), [3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(circuits_after_n_edges(add_n_shortest_edges(parse_input(input), 3)), [3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(circuits_after_n_edges(add_n_shortest_edges(parse_input(input), 4)), [3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(circuits_after_n_edges(add_n_shortest_edges(parse_input(input), 10)), [5, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1]);
    }
}
