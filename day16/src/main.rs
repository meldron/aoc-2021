use anyhow::Result;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
struct ValuePacket {
    pub version: usize,
    pub value: usize,
    pub len: usize,
}

impl ValuePacket {
    fn new(version: usize, raw: &str) -> Self {
        let mut value_raw: Vec<&str> = Vec::new();

        let mut i: usize = 0;
        loop {
            let start = i * 5;
            let end = i * 5 + 5;
            let v = &raw[start..end];

            if v.len() != 5 {
                panic!("value packet parsing error");
            }

            value_raw.push(&v[1..5]);

            i += 1;

            if &v[0..1] == "0" {
                break;
            }
        }

        let value = binary_to_usize(value_raw.join("").as_str());
        let len = i * 5 + 6;

        Self {
            version,
            value,
            len,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum OpType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
    Value,
}

impl From<usize> for OpType {
    fn from(u: usize) -> Self {
        match u {
            0 => OpType::Sum,
            1 => OpType::Product,
            2 => OpType::Minimum,
            3 => OpType::Maximum,
            4 => OpType::Value,
            5 => OpType::GreaterThan,
            6 => OpType::LessThan,
            7 => OpType::EqualTo,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct OperatorPacket {
    pub version: usize,
    pub op_type: OpType,
    pub sub_packets: Vec<Packet>,
    pub len: usize,
}

impl OperatorPacket {
    fn new(version: usize, op_type: OpType, raw: &str) -> Self {
        let length_type_id = &raw[0..1];
        let mut sub_packets = Vec::new();
        let mut total_size = 1;

        match length_type_id {
            "1" => {
                let number_sub_packets = binary_to_usize(&raw[1..12]);
                total_size += 11;
                let mut start: usize = 12;
                for _ in 0..number_sub_packets {
                    let packet = parse_packet(&raw[start..]);

                    total_size += packet.len();
                    start += packet.len();

                    sub_packets.push(packet);
                }
            }
            "0" => {
                let sub_packets_length = binary_to_usize(&raw[1..16]);
                total_size += 15;
                let mut sub_packages_length_counter: usize = 0;
                let mut start: usize = 16;

                while sub_packets_length != sub_packages_length_counter {
                    let packet = parse_packet(&raw[start..]);

                    start += packet.len();
                    sub_packages_length_counter += packet.len();
                    total_size += packet.len();

                    sub_packets.push(packet);
                }
            }
            _ => unreachable!(),
        }

        let len = total_size + 6;

        Self {
            version,
            op_type,
            sub_packets,
            len,
        }
    }

    pub fn version_sum(&self) -> usize {
        let sub_sum: usize = self.sub_packets.iter().map(|p| p.version_sum()).sum();

        sub_sum + self.version as usize
    }

    pub fn value(&self) -> usize {
        let compare = |op: OpType| -> usize {
            let v1 = self.sub_packets[0].value();
            let v2 = self.sub_packets[1].value();

            let b = match op {
                OpType::GreaterThan => v1 > v2,
                OpType::LessThan => v1 < v2,
                OpType::EqualTo => v1 == v2,
                _ => unreachable!(),
            };

            b as usize
        };

        match self.op_type {
            OpType::Sum => self.sub_packets.iter().map(|v| v.value()).sum(),
            OpType::Product => self.sub_packets.iter().fold(1, |acc, p| acc * p.value()),
            OpType::Minimum => self.sub_packets.iter().map(|v| v.value()).min().unwrap(),
            OpType::Maximum => self.sub_packets.iter().map(|v| v.value()).max().unwrap(),
            _ => compare(self.op_type),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum Packet {
    Value(ValuePacket),
    Operator(OperatorPacket),
}

impl Packet {
    pub fn version_sum(&self) -> usize {
        match self {
            Packet::Value(v) => v.version as usize,
            Packet::Operator(o) => o.version_sum(),
        }
    }

    pub fn value(&self) -> usize {
        match self {
            Packet::Value(v) => v.value,
            Packet::Operator(o) => o.value(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Packet::Value(v) => v.len,
            Packet::Operator(o) => o.len,
        }
    }
}

fn binary_to_usize(b: &str) -> usize {
    usize::from_str_radix(b, 2).unwrap()
}

fn parse_packet(input: &str) -> Packet {
    let version = binary_to_usize(&input[0..3]);
    let op_type_raw = binary_to_usize(&input[3..6]);
    let op_type = OpType::from(op_type_raw);

    if op_type == OpType::Value {
        let packet = ValuePacket::new(version, &input[6..]);
        return Packet::Value(packet);
    }

    let packet = OperatorPacket::new(version, op_type, &input[6..]);
    Packet::Operator(packet)
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(""),
    }
}

fn hex_decode(s: &str) -> String {
    s.trim().chars().map(|c| to_binary(c)).collect()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let decoded = hex_decode(&input);

    let packet = parse_packet(&decoded);

    println!("P1: {}", packet.version_sum());
    println!("P2: {}", packet.value());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex_decode_working() {
        let encoded = "D2FE28";
        let decoded = "110100101111111000101000";

        assert_eq!(hex_decode(encoded), decoded)
    }

    #[test]
    fn parse_value_packet() {
        let encoded = "D2FE28";
        let decoded = hex_decode(encoded);

        let packet = parse_packet(&decoded);
        assert_eq!(
            packet,
            Packet::Value(ValuePacket {
                version: 6,
                value: 2021,
                len: 21
            })
        )
    }

    #[test]
    fn parse_op_0() {
        let encoded = "38006F45291200";
        let decoded = hex_decode(encoded);

        let packet = parse_packet(&decoded);
        assert_eq!(
            packet,
            Packet::Operator(OperatorPacket {
                len: 49,
                op_type: OpType::LessThan,
                version: 1,
                sub_packets: vec![
                    Packet::Value(ValuePacket {
                        version: 6,
                        value: 10,
                        len: 11
                    }),
                    Packet::Value(ValuePacket {
                        version: 2,
                        value: 20,
                        len: 16
                    })
                ]
            })
        )
    }

    #[test]
    fn parse_op_1() {
        let encoded = "EE00D40C823060";
        let decoded = hex_decode(encoded);

        let packet = parse_packet(&decoded);
        assert_eq!(
            packet,
            Packet::Operator(OperatorPacket {
                len: 51,
                op_type: OpType::Maximum,
                version: 7,
                sub_packets: vec![
                    Packet::Value(ValuePacket {
                        version: 2,
                        value: 1,
                        len: 11
                    }),
                    Packet::Value(ValuePacket {
                        version: 4,
                        value: 2,
                        len: 11
                    }),
                    Packet::Value(ValuePacket {
                        version: 1,
                        value: 3,
                        len: 11
                    })
                ]
            })
        )
    }
}
