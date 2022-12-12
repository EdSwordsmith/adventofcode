use std::{
    collections::{HashSet, VecDeque},
    io,
};

#[derive(Debug)]
enum Node {
    Start,
    End,
    Middle(u8),
}

impl Node {
    fn height(&self) -> u8 {
        match self {
            Node::Start => 0,
            Node::End => 25,
            Node::Middle(h) => *h,
        }
    }

    fn parse(c: char) -> Node {
        match c {
            'S' => Node::Start,
            'E' => Node::End,
            _ => {
                let mut byte = [0; 1];
                c.encode_utf8(&mut byte);
                Node::Middle(byte[0] - 97)
            }
        }
    }

    fn can_climb_to(&self, to: &Node) -> bool {
        let to_height = to.height();
        let from_height = self.height();
        to_height <= from_height || to_height - from_height == 1
    }
}

#[derive(Debug)]
struct HeightMap {
    nodes: Vec<Vec<Node>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl HeightMap {
    fn parse() -> HeightMap {
        let nodes: Vec<Vec<Node>> = io::stdin()
            .lines()
            .map(|line| line.unwrap().chars().map(Node::parse).collect())
            .collect();

        let start = nodes
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, node)| matches!(node, Node::Start))
                    .map(move |(j, _)| (i, j))
            })
            .next()
            .unwrap();

        let end = nodes
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, node)| matches!(node, Node::End))
                    .map(move |(j, _)| (i, j))
            })
            .next()
            .unwrap();

        HeightMap { nodes, start, end }
    }

    fn adjacent<G: Fn(&Node, &Node) -> bool>(
        &self,
        pos: (usize, usize),
        can_climb: G,
    ) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        let height = self.nodes.len() - 1;
        let width = self.nodes[0].len() - 1;

        if pos.0 != 0 {
            positions.push((pos.0 - 1, pos.1));
        }

        if pos.1 != 0 {
            positions.push((pos.0, pos.1 - 1));
        }

        if pos.0 != height {
            positions.push((pos.0 + 1, pos.1));
        }

        if pos.1 != width {
            positions.push((pos.0, pos.1 + 1));
        }

        let node = &self.nodes[pos.0][pos.1];
        positions.retain(|(row, col)| can_climb(node, &self.nodes[*row][*col]));

        positions
    }

    fn bfs<F: Fn(&HeightMap, (usize, usize)) -> bool, G: Fn(&Node, &Node) -> bool>(
        &self,
        start: (usize, usize),
        reached_end: F,
        can_climb: G,
    ) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((start, 0));
        visited.insert(start);

        while !queue.is_empty() {
            let (pos, dist) = queue.pop_front().unwrap();

            for p in self.adjacent(pos, &can_climb) {
                if visited.contains(&p) {
                    continue;
                }

                if reached_end(self, p) {
                    return dist + 1;
                }

                visited.insert(p);
                queue.push_back((p, dist + 1));
            }
        }

        0
    }
}

fn main() {
    let height_map = HeightMap::parse();
    println!(
        "Part 1: {}",
        height_map.bfs(
            height_map.start,
            |hm, p| p == hm.end,
            |n1, n2| n1.can_climb_to(n2)
        )
    );
    println!(
        "Part 2: {}",
        height_map.bfs(
            height_map.end,
            |hm, p| hm.nodes[p.0][p.1].height() == 0,
            |n1, n2| n2.can_climb_to(n1)
        )
    );
}
