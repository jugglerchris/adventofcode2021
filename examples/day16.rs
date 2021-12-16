use adventofcode2021::get_input;

#[derive(Clone, Debug)]
struct BitStream {
    bytes: Vec<u8>,
    pos: usize,
}

impl BitStream {
    pub fn from_hex(s: &str) -> BitStream {
        let mut bytes = Vec::new();
        for i in 0..(s.len()/2) {
            bytes.push(u8::from_str_radix(&s[i*2..i*2+2], 16).unwrap());
        }
        BitStream {
            bytes,
            pos: 0,
        }
    }

    pub fn get_pos(&self) -> usize {
        self.pos
    }
    pub fn read_bit(&mut self) -> usize {
        let byte = self.bytes[self.pos/8] as usize;
        let result = (byte >> (7-(self.pos%8))) & 1;
        self.pos += 1;
        result
    }

    pub fn read_bits(&mut self, bits: usize) -> usize {
        let mut result = 0;
        for _ in 0..bits {
            result = (result << 1) | self.read_bit();
        }
        result
    }
}

#[test]
fn test_bitstream() {
    let mut bs = BitStream::from_hex("D2FE28");
    assert_eq!(bs.read_bits(3), 0b110);
    assert_eq!(bs.read_bits(3), 0b100);
    assert_eq!(bs.read_bits(5), 0b10111);
    assert_eq!(bs.read_bits(5), 0b11110);
    assert_eq!(bs.read_bits(5), 0b00101);
}

#[derive(Debug)]
enum Payload {
    Literal(usize),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    payload: Payload,
}

fn parse_packet(bs: &mut BitStream, depth: usize) -> Packet {
    dbg!((depth, bs.get_pos()));
    let version = dbg!(bs.read_bits(3));
    let type_id = dbg!(bs.read_bits(3));
    match type_id {
        4 => {
            let mut v = 0;
            loop {
                let piece = bs.read_bits(5);
                v = (v << 4) | piece & 0xF;
                if piece & 0x10 == 0 {
                    break;
                }
            }
            let payload = Payload::Literal(v);
            dbg!(Packet { version, type_id, payload })
        }
        _ => {
            let mode = bs.read_bits(1);
            let mut sub_packets = Vec::new();
            if dbg!(mode) == 0 {
                let sub_packets_length = bs.read_bits(15);
                let end_pos = bs.get_pos() + sub_packets_length;
                while dbg!(bs.get_pos()) < dbg!(end_pos) {
                    sub_packets.push(parse_packet(bs, depth+1));
                }
                assert_eq!(bs.get_pos(), end_pos);
            } else {
                let num_sub_packets = bs.read_bits(11);
                for _ in 0..dbg!(num_sub_packets) {
                    sub_packets.push(parse_packet(bs, depth+1));
                }
            }
            let payload = Payload::Operator(sub_packets);
            dbg!(Packet { version, type_id, payload })
        }
    }
}

fn add_versions(packet: &Packet) -> usize {
    let mut sum = packet.version;
    if let Payload::Operator(sub_packets) = &packet.payload {
        sum += sub_packets.iter()
            .map(add_versions)
            .sum::<usize>();
    }
    sum
}

fn part1(data: &str) -> usize {
    let mut bs = BitStream::from_hex(data);
    let packet = parse_packet(&mut bs, 0);

    add_versions(&packet)
}
fn part2(data: &[u32]) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let test1 = r#"8A004A801A8002F478"#;
    let test2 = r#"620080001611562C8802118E34"#;
    let test3 = r#"C0015000016115A2E0802F182340"#;
    let test4 = r#"A0016C880162017C3686B18A3D4780"#;

    assert_eq!(part1(&test1), 16);
    assert_eq!(part1(&test2), 12);
    assert_eq!(part1(&test3), 23);
    assert_eq!(part1(&test4), 31);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(16)?;

    // Part 1
    println!("{}", part1(&input));

    // Part 2
//    println!("{}", part2(&data));

    Ok(())
}
