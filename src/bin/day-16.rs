use aoc2021::commons::io::load_argv_lines;
use bitvec::prelude::*;
use std::cmp::Ordering;

#[derive(Debug)]
struct BitReader {
    bits: BitVec<Msb0, usize>,
    write_pos: usize,
    read_pos: usize,
}

impl BitReader {
    pub fn new(size: usize) -> Self {
        let mut bits = BitVec::new();
        bits.resize(size, false);
        Self {
            bits,
            write_pos: 0,
            read_pos: 0,
        }
    }

    pub fn push(&mut self, d: u8, size: usize) {
        self.bits[self.write_pos..self.write_pos + size].store(d);
        self.write_pos += size;
    }

    pub fn read(&mut self, size: usize) -> u8 {
        let d = self.bits[self.read_pos..self.read_pos + size].load_be();
        self.read_pos += size;
        d
    }

    pub fn read_to_reader(&mut self, size: usize) -> Self {
        let mut reader = BitReader::new(size);
        let mut remaining = size;
        while remaining > 8 {
            reader.push(self.read(8), 8);
            remaining -= 8;
        }
        reader.push(self.read(remaining), remaining);
        reader
    }

    pub fn has_at_least(&self, size: usize) -> bool {
        (self.read_pos + size) < self.bits.len()
    }
}

#[derive(Debug)]
enum Packet {
    Literal(u8, u128),
    Command(u8, u8, Vec<Packet>),
}

impl Packet {
    pub fn version_sum(&self) -> u128 {
        match self {
            Packet::Literal(v, _) => *v as u128,
            Packet::Command(v, _, ref sub) => {
                *v as u128 + sub.iter().map(|p| p.version_sum() as u128).sum::<u128>()
            }
        }
    }

    pub fn eval(&self) -> u128 {
        match self {
            Packet::Literal(_, val) => *val,
            Packet::Command(_, op, ref sub) => {
                let mut vals = sub.iter().map(|p| p.eval());
                match op {
                    0 => vals.sum(),
                    1 => vals.product(),
                    2 => vals.min().unwrap(),
                    3 => vals.max().unwrap(),
                    5 | 6 | 7 => {
                        let a = vals.next().unwrap();
                        let b = vals.next().unwrap();
                        let c = a.cmp(&b);
                        match (op, c) {
                            (5, Ordering::Greater) => 1,
                            (6, Ordering::Less) => 1,
                            (7, Ordering::Equal) => 1,
                            _ => 0,
                        }
                    }
                    _ => panic!("Unknown op {}", op),
                }
            }
        }
    }
}

fn read_packet(reader: &mut BitReader) -> Packet {
    let version = reader.read(3);
    let type_id = reader.read(3);
    match type_id {
        4 => {
            // Literal value
            let mut more = true;
            let mut value: u128 = 0;
            while more {
                let part = reader.read(5);
                more = (part & 0b10000) != 0;
                value = value << 4 | (part & 0b01111) as u128;
            }
            Packet::Literal(version, value)
        }
        _ => {
            // Command
            let length_type_id = reader.read(1);
            let subpackets = if length_type_id == 0 {
                // Total length of packet in bits
                let mut length = reader.read(7) as usize;
                length = (length << 8) | reader.read(8) as usize;
                let mut subreader = reader.read_to_reader(length);
                read_packets(&mut subreader)
            } else {
                let mut length = reader.read(3) as usize;
                length = (length << 8) | reader.read(8) as usize;
                let mut packets = Vec::new();
                for _ in 0..length {
                    packets.push(read_packet(reader));
                }
                packets
            };
            Packet::Command(version, type_id, subpackets)
        }
    }
}

fn read_packets(reader: &mut BitReader) -> Vec<Packet> {
    let mut packets = Vec::new();

    while reader.has_at_least(7) {
        let packet = read_packet(reader);
        packets.push(packet);
    }

    packets
}

fn main() {
    let input = load_argv_lines::<String>().next().unwrap().unwrap();
    let mut reader = BitReader::new(input.len() * 4);
    for c in input.chars() {
        let d = c.to_digit(16).unwrap();
        reader.push(d as u8, 4);
    }
    let packet = read_packet(&mut reader);
    println!("{}", packet.version_sum());
    println!("{}", packet.eval());
}
