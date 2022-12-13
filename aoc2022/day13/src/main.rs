use std::{cmp::Ordering, io};

use parser::PacketData;

mod parser;
mod scanner;

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketData::Int(v1), PacketData::Int(v2)) => v1.cmp(v2),
            (PacketData::List(items1), PacketData::List(items2)) => {
                for (p1, p2) in items1.iter().zip(items2) {
                    if p1 == p2 {
                        continue;
                    }

                    return p1.cmp(p2);
                }

                if items1.len() < items2.len() {
                    Ordering::Less
                } else if items1.len() > items2.len() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (PacketData::Int(v1), PacketData::List(_)) => {
                let list = PacketData::List(vec![PacketData::Int(*v1)]);
                list.cmp(other)
            }
            (PacketData::List(_), PacketData::Int(v2)) => {
                let list = PacketData::List(vec![PacketData::Int(*v2)]);
                self.cmp(&list)
            }
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for PacketData {}

impl PacketData {
    fn is_divider_packet(&self) -> bool {
        if let PacketData::List(inner) = self {
            if inner.len() == 1 {
                if let PacketData::List(inner) = &inner[0] {
                    if inner.len() == 1 {
                        if let PacketData::Int(n) = &inner[0] {
                            return *n == 2 || *n == 6;
                        }
                    }
                }
            }
        }

        false
    }
}

fn main() {
    let mut packets: Vec<PacketData> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.trim().is_empty())
        .map(|source| scanner::scan_tokens(source))
        .map(|tokens| parser::parse(tokens))
        .collect();

    let pairs_in_right_order: usize = packets
        .windows(2)
        .step_by(2)
        .enumerate()
        .filter(|(_, packets)| packets[0] < packets[1])
        .map(|(i, _)| i + 1)
        .sum();

    println!("Part 1: {}", pairs_in_right_order);

    packets.push(PacketData::List(vec![PacketData::List(vec![PacketData::Int(2)])]));
    packets.push(PacketData::List(vec![PacketData::List(vec![PacketData::Int(6)])]));
    packets.sort();

    let decoder_key: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, p)| p.is_divider_packet())
        .map(|(i, _)| i + 1)
        .product();
    
    println!("Part 2: {}", decoder_key);
}
