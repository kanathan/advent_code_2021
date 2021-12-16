use std::fmt::Debug;



fn main() {
    let input = include_str!("input");

    println!("{}",get_version_sum(input));

    println!("{}",get_packet_val(input));
}

fn get_packet_val(input: &str) -> u64 {
    let mut bin = get_binary(input);
    consume_packet(&mut bin).val
}

fn get_version_sum(input: &str) -> u32 {
    let mut bin = get_binary(input);
    let packet = consume_packet(&mut bin);

    fn get_versions(packet: &Packet) -> Vec<u8> {
        let mut versions = vec![packet.version];
        for sub_packet in packet.sub_packets.iter() {
            versions.append(&mut get_versions(sub_packet));
        }
        versions
    }

    get_versions(&packet)
        .iter()
        .map(|v| *v as u32)
        .sum()
}

fn get_binary(input: &str) -> String {
    input
        .replace("\n", "")
        .chars()
        .map(|c| format!("{:04b}",c.to_digit(16).unwrap()))
        .collect()
}

fn consume_packet(bin: &mut String) -> Packet {
    let packet;

    let version = u8::from_str_radix(&bin.drain(..3).collect::<String>(), 2).unwrap();
    let id = u8::from_str_radix(&bin.drain(..3).collect::<String>(), 2).unwrap();

    if id == 4 { // Literal value packet
        let mut bit_value = String::new();
        loop {
            let bit_group = bin.drain(..5).collect::<String>();
            bit_value.push_str(&bit_group[1..]);
            if bit_group.chars().next().unwrap() == '0' { break }
        }
        let val = u64::from_str_radix(&bit_value, 2).expect(&format!("Unable to parse: {}",bit_value));
        packet = Packet { version, id, ty: PacketType::Literal, val, sub_packets: Vec::new() };
    }
    else { // Operator packet
        let mut sub_packets = Vec::new();
        let mode = bin.drain(..1).next().unwrap();
        match mode {
            '0' => {
                let subpacket_len = usize::from_str_radix(&bin.drain(..15).collect::<String>(), 2).unwrap();
                let mut subpacket_str = bin.drain(..subpacket_len).collect::<String>();
                while !subpacket_str.is_empty() {
                    sub_packets.push(consume_packet(&mut subpacket_str));
                }
            },
            '1' => {
                let subpacket_cnt = usize::from_str_radix(&bin.drain(..11).collect::<String>(), 2).unwrap();
                for _ in 0..subpacket_cnt {
                    sub_packets.push(consume_packet(bin));
                }
            }
            _ => panic!("Invalid mode")
        }

        let val = match id {
            0 => sub_packets.iter().map(|p| p.val).sum(),
            1 => sub_packets.iter().map(|p| p.val).product(),
            2 => sub_packets.iter().map(|p| p.val).min().unwrap(),
            3 => sub_packets.iter().map(|p| p.val).max().unwrap(),
            5 => if sub_packets[0].val > sub_packets[1].val { 1 } else { 0 },
            6 => if sub_packets[0].val < sub_packets[1].val { 1 } else { 0 },
            7 => if sub_packets[0].val == sub_packets[1].val { 1 } else { 0 },
            _ => panic!("Invalid ID: {}",id)
        };

        packet = Packet { version, id, ty: PacketType::Operator, val, sub_packets};
    }

    packet
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Packet {
    version: u8,
    id: u8,
    ty: PacketType,
    val: u64,
    sub_packets: Vec<Packet>
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Packet")
        .field("version", &self.version)
        .field("id", &self.id)
        .field("ty", &self.ty)
        .field("val", &self.val)
        .field("sub_packets", &self.sub_packets.len())
        .finish()
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum PacketType {
    Operator,
    Literal,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(get_binary("D2FE28"), "110100101111111000101000");
        assert_eq!(get_binary("38006F45291200"), "00111000000000000110111101000101001010010001001000000000");
        assert_eq!(get_binary("EE00D40C823060"), "11101110000000001101010000001100100000100011000001100000");
    }

    #[test]
    fn test2() {
        assert_eq!(get_version_sum("8A004A801A8002F478"), 16);
        assert_eq!(get_version_sum("620080001611562C8802118E34"), 12);
        assert_eq!(get_version_sum("C0015000016115A2E0802F182340"), 23);
        assert_eq!(get_version_sum("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test3() {
        let mut bin = get_binary("9C0141080250320F1802104A08");
        println!("{:?}",consume_packet(&mut bin));
    }
}