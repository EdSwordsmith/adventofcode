use std::env;
use std::fs;

struct PacketReader<'a> {
    bytes: &'a [u8],
    current: usize,
    bits: u8,
    read: u64,
}

impl<'a> PacketReader<'a> {
    fn new(bytes: &[u8]) -> PacketReader {
        PacketReader {
            bytes,
            current: 0,
            bits: 0,
            read: 0,
        }
    }

    fn read_n_bits(&mut self, n: u32) -> u32 {
        let mut bits = n;
        let mut res = 0;

        while bits > 0 {
            let b = bits.min((8 - self.bits) as u32) as u8;
            let shift = (8 - self.bits) - b;
            let mask = (0b11111111 >> self.bits + shift) << shift;
            res = res << b;
            res = res | (((self.bytes[self.current] & mask) >> shift) as u32);

            bits -= b as u32;
            self.read += b as u64;
            self.bits = (self.bits + b) % 8;
            if self.bits == 0 {
                self.current += 1;
            }
        }

        res
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

#[derive(Debug)]
enum PacketType {
    LitValue { value: u128 },
    Operator { op: u8, packets: Vec<Packet> },
}

impl PacketType {
    fn parse(reader: &mut PacketReader) -> PacketType {
        let packet_type = reader.read_n_bits(3) as u8;

        match packet_type {
            4 => {
                let mut value: u128 = 0;

                loop {
                    let cont = reader.read_n_bits(1);
                    let hbyte = reader.read_n_bits(4) as u128;
                    value = (value << 4) | hbyte;

                    if cont == 0 {
                        break;
                    }
                }

                PacketType::LitValue { value }
            }
            _ => {
                let mut packets = vec![];
                let length_type = reader.read_n_bits(1);

                if length_type == 0 {
                    let bits = reader.read_n_bits(15) as u64;
                    let target_read = reader.read + bits;
                    while reader.read != target_read {
                        let packet = Packet::parse(reader);
                        packets.push(packet);
                    }
                } else {
                    let npackets = reader.read_n_bits(11);
                    for _ in 0..npackets {
                        let packet = Packet::parse(reader);
                        packets.push(packet);
                    }
                }

                PacketType::Operator {
                    op: packet_type as u8,
                    packets,
                }
            }
        }
    }
}

impl Packet {
    fn parse(reader: &mut PacketReader) -> Packet {
        let version = reader.read_n_bits(3) as u8;
        Packet {
            version,
            packet_type: PacketType::parse(reader),
        }
    }

    fn versions_sum(&self) -> u32 {
        let sum = match &self.packet_type {
            PacketType::Operator { op: _, packets } => {
                packets.iter().map(|packet| packet.versions_sum()).sum()
            }
            _ => 0,
        };

        self.version as u32 + sum
    }

    fn eval(&self) -> u128 {
        match &self.packet_type {
            PacketType::LitValue { value } => *value,
            PacketType::Operator { op, packets } => match *op {
                0 => packets.iter().map(|p| p.eval()).sum(),
                1 => packets.iter().map(|p| p.eval()).product(),
                2 => packets.iter().map(|p| p.eval()).min().unwrap(),
                3 => packets.iter().map(|p| p.eval()).max().unwrap(),
                5 => {
                    if packets[0].eval() > packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].eval() < packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].eval() == packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!("Invalid operation"),
            },
        }
    }
}

fn main() {
    let bytes = read_hex_input();
    let mut reader = PacketReader::new(&bytes);
    let packet = Packet::parse(&mut reader);

    // println!("{:?}", packet);
    println!("Part 1: {}", packet.versions_sum());
    println!("Part 2: {}", packet.eval());
}

fn read_hex_input() -> Vec<u8> {
    let input_path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(input_path).unwrap();
    (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
        .collect()
}
