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
        Cave {
            big,
            start,
            end,
        }
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
    paths: u32,
    paths2: HashSet<Vec<usize>>,
}

impl CavesGraph {
    fn new() -> Self {
        Self {
            by_name: HashMap::new(),
            caves: Vec::new(),
            graph: Vec::new(),
            paths: 0,
            paths2: HashSet::new(),
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

    fn solve(&mut self) {
        let start = self.add_or_get(&String::from("start"));
        let mut visited = Vec::new();
        self.paths = 0;
        for _ in self.caves.iter() {
            visited.push(false);
        }

        self.dfs(start, visited);
    }

    fn dfs(&mut self, index: usize, mut visited: Vec<bool>) {
        //println!("{} - {:?}", index, visited);

        if !self.caves[index].big {
            visited[index] = true;
        }

        for adj in self.graph[index].clone().iter() {
            if !visited[*adj] {
                self.dfs(*adj, visited.clone());
            }

            if self.caves[*adj].end {
                self.paths += 1;
            }
        }
    }

    fn solve2(&mut self) {
        let start = self.add_or_get(&String::from("start"));
        let end = self.add_or_get(&String::from("end"));
        let mut visited = Vec::new();
        self.paths2.clear();
        for _ in self.caves.iter() {
            visited.push(0);
        }
        visited[start] = 1;
        visited[end] = 1;

        self.dfs2(start, visited, Vec::new());
        self.paths2.len();
    }

    fn dfs2(&mut self, index: usize, mut visited: Vec<u32>, mut path: Vec<usize>) {
        path.push(index);

        if self.caves[index].end {
            println!("{} - {:?}", index, path);
            self.paths2.insert(path);
            return;
        }

        if !self.caves[index].big {
            visited[index] += 1;
        }

        for adj in self.graph[index].clone().iter() {
            if visited[*adj] < 2 {
                self.dfs2(*adj, visited.clone(), path.clone());
            }
        }
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

    graph.solve();
    println!("{}", graph.paths);
    graph.solve2();
    println!("{}", graph.paths);

    //let this_isnt_gonna_work = BFS(&by_name, &mut caves, &graph);
    //print!("{}", this_isnt_gonna_work);
}
