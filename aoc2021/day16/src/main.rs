use std::env;
use std::fs;
use std::str::Chars;
fn main() {
    let binary = get_binary();
    let mut bits = binary.chars();
    let mut version_sum = 0;
    let mut value = 0;
    parse_packet(&mut bits, &mut version_sum, &mut value);
    println!("Part 1: {}", version_sum);
    println!("Part 2: {}", value);
}

fn parse_packet(bits: &mut Chars, version_sum: &mut u64, value: &mut u64) -> u64 {
    *value = 0;
    let version = get_n_bits(3, bits);
    let type_id = get_n_bits(3, bits);

    let mut bits_read = 6;
    *version_sum += version;

    if type_id != 4 {
        bits_read += 1;
        let length_type = get_n_bits(1, bits);

        if length_type == 0 {
            let subpackets_bits = get_n_bits(15, bits);
            let mut read = 0;
            bits_read += 15;
            let mut first = true;
            let mut prev = 0;

            while subpackets_bits > read {
                let mut v = 0;
                read += parse_packet(bits, version_sum, &mut v);
                if type_id == 0 {
                    *value += v;
                } else if type_id == 1 {
                    *value *= v;
                } else if type_id == 2 {
                    if v < *value {
                        *value = v;
                    }
                } else if type_id == 3 {
                    if v > *value {
                        *value = v;
                    }
                } else if type_id == 5 {
                    if first {
                        prev = v;
                    } else {
                        *value = if v < prev { 1 } else { 0 };
                    }
                } else if type_id == 6 {
                    if first {
                        prev = v;
                    } else {
                        *value = if v > prev { 1 } else { 0 };
                    }
                } else if type_id == 7 {
                    if first {
                        prev = v;
                    } else {
                        *value = if v == prev { 1 } else { 0 };
                    }
                }

                first = false;
            }
        } else {
            let subpackets_count = get_n_bits(11, bits);
            let mut prev = 0;
            bits_read += 11;
            for i in 0..subpackets_count {
                let mut v = 0;
                bits_read += parse_packet(bits, version_sum, &mut v);
                if type_id == 0 {
                    *value += v;
                } else if type_id == 1 {
                    *value *= v;
                } else if type_id == 2 {
                    if v < *value {
                        *value = v;
                    }
                } else if type_id == 3 {
                    if v > *value {
                        *value = v;
                    }
                } else if type_id == 5 {
                    if i == 0 {
                        prev = v;
                    } else {
                        *value = if v < prev { 1 } else { 0 };
                    }
                } else if type_id == 6 {
                    if i == 0 {
                        prev = v;
                    } else {
                        *value = if v > prev { 1 } else { 0 };
                    }
                } else if type_id == 7 {
                    if i == 0 {
                        prev = v;
                    } else {
                        *value = if v == prev { 1 } else { 0 };
                    }
                }
            }
        }
    } else {
        let mut nextbits = get_n_bits(5, bits);
        bits_read += 5;
        let mut keep_going = nextbits & 16 != 0;
        *value = *value * 16 + (nextbits & 15);

        while keep_going {
            nextbits = get_n_bits(5, bits);
            keep_going = nextbits & 16 != 0;
            *value = *value * 16 + (nextbits & 15);
            bits_read += 5;
        }
    }

    bits_read
}

fn get_binary() -> String {
    let input_path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(input_path).unwrap();
    let mut binary = String::new();

    for c in input.chars() {
        let value = c.to_digit(16).unwrap();
        let mut hbyte = format!("{:b}", value);
        while hbyte.len() < 4 {
            hbyte = String::from("0") + hbyte.as_str();
        }
        binary += hbyte.as_str();
    }

    binary
}

fn get_n_bits(n: usize, bits: &mut Chars) -> u64 {
    let mut value = 0;
    for _ in 0..n {
        if let Some(bit) = bits.next() {
            let bit = bit.to_digit(2).unwrap();
            value = value * 2 + bit;
        } else {
            break;
        }
    }

    value as u64
}
