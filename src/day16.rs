use std::io::BufRead;

use bitvec::prelude::*;

const DATA_FILE: &str = "16.txt";

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> impl Iterator<Item = String> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file).unwrap());
    data.lines().map(|s_res| s_res.unwrap())
}

pub fn parse<I: Iterator<Item = String>>(mut data: I) -> Vec<u8> {
    let data = data.next().unwrap();
    data.as_bytes()
        .chunks(2)
        .map(|b| std::str::from_utf8(b).unwrap())
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    symbol: Symbol,
}

impl Packet {
    fn parse_group(bits_in: &BitSlice<Msb0, u8>) -> (Vec<Packet>, usize) {
        let mut out: Vec<Packet> = Vec::with_capacity(2);
        let mut offset = 0;

        while offset < bits_in.len() && bits_in[offset..].any() {
            let (packet, size) = Packet::parse(&bits_in[offset..]);
            out.push(packet);
            //println!("Parsed packet at offset {}, moving to next packet at {}", offset, offset + size);
            offset += size;
        }
        (out, offset)
    }

    fn parse_group_n(bits_in: &BitSlice<Msb0, u8>, max_packets: usize) -> (Vec<Packet>, usize) {
        let mut out: Vec<Packet> = Vec::with_capacity(max_packets);
        let mut offset = 0;

        while offset < bits_in.len() && out.len() < max_packets {
            let (packet, size) = Packet::parse(&bits_in[offset..]);
            out.push(packet);
            offset += size;
        }
        (out, offset)
    }

    fn parse(bits_in: &BitSlice<Msb0, u8>) -> (Packet, usize) {
        //println!("Parsing packet from bits {:?}", bits_in);
        let mut count = 0;
        let version = bits_in[count..count + 3].load_be::<u8>();
        count += 3;
        //println!("Found packet version {}", version);
        let (symbol, size) = Symbol::parse(&bits_in[count..]);
        count += size;
        (Packet { version, symbol }, count)
    }

    fn eval(&self) -> u64 {
        self.symbol.eval()
    }
}

#[derive(Debug, Clone)]
enum Symbol {
    Literal(u64),
    OperatorSum(Vec<Packet>),
    OperatorProduct(Vec<Packet>),
    OperatorMin(Vec<Packet>),
    OperatorMax(Vec<Packet>),
    OperatorGt(Vec<Packet>),
    OperatorLt(Vec<Packet>),
    OperatorEq(Vec<Packet>),
}

impl Symbol {
    fn parse(bits_in: &BitSlice<Msb0, u8>) -> (Self, usize) {
        let mut count = 0;
        let (type_id_bits, bits_in) = bits_in.split_at(3);
        count += 3;
        let type_id = type_id_bits.load_be::<u8>();
        //println!("\tFound packet type {:#x}", type_id);
        match type_id {
            0x4 => {
                let (sym, len) = Symbol::parse_literal(bits_in);
                (sym, count + len)
            }
            op => {
                let (sym, len) = Symbol::parse_op(bits_in, op);
                (sym, count + len)
            }
        }
    }

    fn parse_literal(bits_in: &BitSlice<Msb0, u8>) -> (Self, usize) {
        //println!("\tparse_literal({:?})", bits_in);
        let (literal, count) = bits_in
            .chunks_exact(5)
            //.inspect(|b| println!("Parsing literal chunk {:b}", b))
            .map(|b| b.load_be::<u8>())
            //.inspect(|b| println!("Chunk became byte {:05b}", b))
            .enumerate()
            .scan((true, 0u64), |(do_next, acc), (count, x)| {
                if *do_next {
                    *do_next = (x & 0b010000) > 0;
                    *acc = (*acc << 4) | (x & 0b01111) as u64;
                    Some((*acc, (count + 1) * 5))
                } else {
                    None
                }
            })
            //.inspect(|l| println!("Literal chunk became {:04b} [{} bits long]", l.0, l.1))
            .last()
            .unwrap();

        //println!("\tFound literal {:016b}, consuming {} bits", literal, count);
        (Symbol::Literal(literal), count)
    }

    fn parse_op(bits_in: &BitSlice<Msb0, u8>, op: u8) -> (Self, usize) {
        let mut count = 0;

        let length_type_id = bits_in[0];
        count += 1;

        let (out, count) = if length_type_id {
            let packet_count_bits = &bits_in[count..count + 11];
            count += 11;
            let packet_count = packet_count_bits.load_be::<usize>();
            //println!("\tPacket count: {} packets", packet_count);
            let (out, more_count) = Packet::parse_group_n(&bits_in[count..], packet_count);
            count += more_count;
            //println!("\tFound op {} with {} children, consuming {} bits", op, out.len(), count);
            (out, count)
        } else {
            let payload_length_bits = &bits_in[count..count + 15];
            //println!("\tPayload length bits: {:b}", payload_length_bits);
            count += 15;
            let payload_length = payload_length_bits.load_be::<usize>();
            //println!("\tPayload length: {} bits", payload_length);
            let (out, _more_count) = Packet::parse_group(&bits_in[count..count + payload_length]);
            // TODO: this is probably off by one right now
            count += payload_length;
            // TODO: assert relationship between more_count and payload_length
            //println!("\tFound op {} with {} children, consuming {} bits", op, out.len(), count);
            (out, count)
        };

        match op {
            0 => (Symbol::OperatorSum(out), count),
            1 => (Symbol::OperatorProduct(out), count),
            2 => (Symbol::OperatorMin(out), count),
            3 => (Symbol::OperatorMax(out), count),
            5 => (Symbol::OperatorGt(out), count),
            6 => (Symbol::OperatorLt(out), count),
            7 => (Symbol::OperatorEq(out), count),
            _ => unreachable!(),
        }
    }

    fn children(&self) -> Vec<Packet> {
        match self {
            Symbol::Literal(_) => Vec::new(),
            Symbol::OperatorSum(children) => children.clone(),
            Symbol::OperatorProduct(children) => children.clone(),
            Symbol::OperatorMin(children) => children.clone(),
            Symbol::OperatorMax(children) => children.clone(),
            Symbol::OperatorGt(children) => children.clone(),
            Symbol::OperatorLt(children) => children.clone(),
            Symbol::OperatorEq(children) => children.clone(),
        }
    }

    fn eval(&self) -> u64 {
        let val = match self {
            Symbol::Literal(u) => *u as u64,
            Symbol::OperatorSum(children) => children.iter().map(|p| p.eval()).sum(),
            Symbol::OperatorProduct(children) => children
                .iter()
                .map(|p| p.eval())
                .reduce(|accum, item| item * accum)
                .unwrap(),
            Symbol::OperatorMin(children) => children.iter().map(|p| p.eval()).min().unwrap(),
            Symbol::OperatorMax(children) => children.iter().map(|p| p.eval()).max().unwrap(),
            Symbol::OperatorGt(children) => (children[0].eval() > children[1].eval()) as u64,
            Symbol::OperatorLt(children) => (children[0].eval() < children[1].eval()) as u64,
            Symbol::OperatorEq(children) => (children[0].eval() == children[1].eval()) as u64,
        };

        //println!("eval({:?}) => {}", self, val);
        val
    }
}

fn version_sum_packet(packet: &Packet) -> u32 {
    let mut sum = packet.version as u32;
    for child in packet.symbol.children() {
        sum += version_sum_packet(&child) as u32;
    }
    sum
}

pub fn star1(data: &[u8]) -> u32 {
    //println!("Scoring {:x?}", data);
    let bits = BitSlice::<Msb0, u8>::from_slice(data).unwrap();
    let (packets, _) = Packet::parse_group(bits);
    let mut ver_sum: u32 = 0;
    for packet in packets {
        //println!("Packet: {:?}", packet);
        ver_sum += version_sum_packet(&packet);
    }
    ver_sum
}

pub fn star2(data: &[u8]) -> u64 {
    //println!("Scoring {:x?}", data);
    let bits = BitSlice::<Msb0, u8>::from_slice(data).unwrap();
    let (packets, _) = Packet::parse_group(bits);
    if let Some(packet) = packets.into_iter().next() {
        packet.eval()
    } else {
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA_1: ([&'static str; 1], u32) = (["D2FE28"], 6);

    const SAMPLE_DATA_2: ([&'static str; 1], u32) = (["38006F45291200"], 9);

    const SAMPLE_DATA_3: ([&'static str; 1], u32) = (["EE00D40C823060"], 14);

    const SAMPLE_DATA_4: ([&'static str; 1], u32) = (["8A004A801A8002F478"], 16);

    const SAMPLE_DATA_5: ([&'static str; 1], u32) = (["620080001611562C8802118E34"], 12);

    const SAMPLE_DATA_6: ([&'static str; 1], u32) = (["C0015000016115A2E0802F182340"], 23);

    const SAMPLE_DATA_7: ([&'static str; 1], u32) = (["A0016C880162017C3686B18A3D4780"], 31);

    #[test]
    fn test_star1() {
        //println!("\n==== Test 1 ====");
        let data = parse(SAMPLE_DATA_1.0.iter().map(|r| r.to_string()));
        assert_eq!(star1(&data), SAMPLE_DATA_1.1);

        //println!("\n==== Test 2 ====");
        let data = parse(SAMPLE_DATA_2.0.iter().map(|r| r.to_string()));
        assert_eq!(star1(&data), SAMPLE_DATA_2.1);

        //println!("\n==== Test 3 ====");
        let data = parse(SAMPLE_DATA_3.0.iter().map(|r| r.to_string()));
        assert_eq!(star1(&data), SAMPLE_DATA_3.1);

        //println!("\n==== Test 4 ====");
        let data = parse(SAMPLE_DATA_4.0.iter().map(|r| r.to_string()));
        assert_eq!(star1(&data), SAMPLE_DATA_4.1);

        //println!("\n==== Test 5 ====");
        let data = parse(SAMPLE_DATA_5.0.iter().map(|r| r.to_string()));
        assert_eq!(star1(&data), SAMPLE_DATA_5.1);

        //println!("\n==== Test 6 ====");
        let data = parse(SAMPLE_DATA_6.0.iter().map(|r| r.to_string()));
        assert_eq!(star1(&data), SAMPLE_DATA_6.1);

        //println!("\n==== Test 7 ====");
        let data = parse(SAMPLE_DATA_7.0.iter().map(|r| r.to_string()));
        assert_eq!(star1(&data), SAMPLE_DATA_7.1);
    }

    const SAMPLE_DATA_A: ([&'static str; 1], u64) = (["C200B40A82"], 3);

    const SAMPLE_DATA_B: ([&'static str; 1], u64) = (["04005AC33890"], 54);

    const SAMPLE_DATA_C: ([&'static str; 1], u64) = (["880086C3E88112"], 7);

    const SAMPLE_DATA_D: ([&'static str; 1], u64) = (["CE00C43D881120"], 9);

    const SAMPLE_DATA_E: ([&'static str; 1], u64) = (["D8005AC2A8F0"], 1);

    const SAMPLE_DATA_F: ([&'static str; 1], u64) = (["F600BC2D8F"], 0);

    const SAMPLE_DATA_G: ([&'static str; 1], u64) = (["9C005AC2F8F0"], 0);

    const SAMPLE_DATA_H: ([&'static str; 1], u64) = (["9C0141080250320F1802104A08"], 1);

    #[test]
    fn test_star2() {
        //println!("\n==== Test A ====");
        let data = parse(SAMPLE_DATA_A.0.iter().map(|r| r.to_string()));
        assert_eq!(star2(&data), SAMPLE_DATA_A.1);

        //println!("\n==== Test B ====");
        let data = parse(SAMPLE_DATA_B.0.iter().map(|r| r.to_string()));
        assert_eq!(star2(&data), SAMPLE_DATA_B.1);

        //println!("\n==== Test C ====");
        let data = parse(SAMPLE_DATA_C.0.iter().map(|r| r.to_string()));
        assert_eq!(star2(&data), SAMPLE_DATA_C.1);

        //println!("\n==== Test D ====");
        let data = parse(SAMPLE_DATA_D.0.iter().map(|r| r.to_string()));
        assert_eq!(star2(&data), SAMPLE_DATA_D.1);

        //println!("\n==== Test E ====");
        let data = parse(SAMPLE_DATA_E.0.iter().map(|r| r.to_string()));
        assert_eq!(star2(&data), SAMPLE_DATA_E.1);

        //println!("\n==== Test F ====");
        let data = parse(SAMPLE_DATA_F.0.iter().map(|r| r.to_string()));
        assert_eq!(star2(&data), SAMPLE_DATA_F.1);

        //println!("\n==== Test G ====");
        let data = parse(SAMPLE_DATA_G.0.iter().map(|r| r.to_string()));
        assert_eq!(star2(&data), SAMPLE_DATA_G.1);

        //println!("\n==== Test H ====");
        let data = parse(SAMPLE_DATA_H.0.iter().map(|r| r.to_string()));
        assert_eq!(star2(&data), SAMPLE_DATA_H.1);
    }
}
