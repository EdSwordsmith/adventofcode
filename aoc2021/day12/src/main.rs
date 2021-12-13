use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Cave {
    big: bool,
    start: bool,
    end: bool,
}

impl Cave {
    fn new(big: bool, start: bool, end: bool) -> Self {
        Cave { big, start, end }
    }

    fn small() -> Self {
        Self::new(false, false, false)
    }

    fn big() -> Self {
        Self::new(true, false, false)
    }

    fn start() -> Self {
        Self::new(false, true, false)
    }

    fn end() -> Self {
        Self::new(false, false, true)
    }
}

struct CavesGraph {
    by_name: HashMap<String, usize>,
    caves: Vec<Cave>,
    graph: Vec<Vec<usize>>,
}

impl CavesGraph {
    fn new() -> Self {
        Self {
            by_name: HashMap::new(),
            caves: Vec::new(),
            graph: Vec::new(),
        }
    }

    fn add_or_get(&mut self, name: &String) -> usize {
        if !self.by_name.contains_key(name) {
            let index = self.caves.len();
            self.by_name.insert(name.clone(), index);
            let start = name.eq("start");
            let end = name.eq("end");
            let big = name.chars().next().unwrap().is_uppercase();

            let cave = if start {
                Cave::start()
            } else if end {
                Cave::end()
            } else if big {
                Cave::big()
            } else {
                Cave::small()
            };

            self.caves.push(cave);
            self.graph.push(Vec::new());
            index
        } else {
            *self.by_name.get(name).unwrap()
        }
    }

    fn add_edge(&mut self, n1: usize, n2: usize) {
        self.graph[n1].push(n2);
    }

    fn solve(&mut self) -> u32 {
        let start = self.add_or_get(&String::from("start"));
        let mut visited = Vec::new();
        for _ in self.caves.iter() {
            visited.push(false);
        }

        self.dfs(start, visited, false, false)
    }

    fn solve2(&mut self) -> u32 {
        let start = self.add_or_get(&String::from("start"));
        let mut visited = Vec::new();
        for _ in self.caves.iter() {
            visited.push(false);
        }
        self.dfs(start, visited, false, true)
    }

    fn dfs(&mut self, index: usize, mut visited: Vec<bool>, mut twice: bool, part2: bool) -> u32 {
        if !self.caves[index].big {
            if visited[index] {
                twice = true;
            }
            visited[index] = true;
        }

        if self.caves[index].end {
            return 1;
        }

        self.graph[index]
            .clone()
            .iter()
            .filter(|i| !visited[**i] || (!twice && part2))
            .map(|i| self.dfs(*i, visited.clone(), twice, part2))
            .sum()
    }
}

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);

    let mut graph = CavesGraph::new();

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (c1, c2) = line.split_once('-').unwrap();
            (c1.to_string(), c2.to_string())
        })
        .fold((), |_, (c1, c2)| {
            let c1_index = graph.add_or_get(&c1);
            let c2_index = graph.add_or_get(&c2);
            if !c1.eq("end") && !c2.eq("start") {
                graph.add_edge(c1_index, c2_index);
            }

            if !c1.eq("start") && !c2.eq("end") {
                graph.add_edge(c2_index, c1_index);
            }
        });

    println!("{}", graph.solve());
    println!("{}", graph.solve2());
}
