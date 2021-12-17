use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::ptr::write;
use std::str::{Chars, FromStr};
use std::string::ParseError;
use std::thread::current;

use itertools::{fold, Itertools};
use phf::phf_map;
use tailcall::tailcall;

const E1: &str = "D2FE28";
const E2: &str = "38006F45291200";
const E3: &str = "EE00D40C823060";
const E4: &str = "8A004A801A8002F478";
const E5: &str = "A0016C880162017C3686B18A3D4780";
const E6: &str = "";
const E7: &str = "";
const INPUT: &str = "A20D5CECBD6C061006E7801224AF251AEA06D2319904921880113A931A1402A9D83D43C9FFCC1E56FF29890E00C42984337BF22C502008C26982801009426937320124E602BC01192F4A74FD7B70692F4A74FD7B700403170400F7002DC00E7003C400B0023700082C601DF8C00D30038005AA0013F40044E7002D400D10030C008000574000AB958B4B8011074C0249769913893469A72200B42673F26A005567FCC13FE673004F003341006615421830200F4608E7142629294F92861A840118F1184C0129637C007C24B19AA2C96335400013B0C0198F716213180370AE39C7620043E0D4788B440232CB34D80260008645C86D16C401B85D0BA2D18025A00ACE7F275324137FD73428200ECDFBEFF2BDCDA70D5FE5339D95B3B6C98C1DA006772F9DC9025B057331BF7D8B65108018092599C669B4B201356763475D00480010E89709E090002130CA28C62300265C188034BA007CA58EA6FB4CDA12799FD8098021400F94A6F95E3ECC73A77359A4EFCB09CEF799A35280433D1BCCA666D5EFD6A5A389542A7DCCC010958D85EC0119EED04A73F69703669466A048C01E14FFEFD229ADD052466ED37BD8B4E1D10074B3FF8CF2BBE0094D56D7E38CADA0FA80123C8F75F9C764D29DA814E4693C4854C0118AD3C0A60144E364D944D02C99F4F82100607600AC8F6365C91EC6CBB3A072C404011CE8025221D2A0337158200C97001F6978A1CE4FFBE7C4A5050402E9ECEE709D3FE7296A894F4C6A75467EB8959F4C013815C00FACEF38A7297F42AD2600B7006A0200EC538D51500010B88919624CE694C0027B91951125AFF7B9B1682040253D006E8000844138F105C0010D84D1D2304B213007213900D95B73FE914CC9FCBFA9EEA81802FA0094A34CA3649F019800B48890C2382002E727DF7293C1B900A160008642B87312C0010F8DB08610080331720FC580";

fn part1() {
    let input: String = INPUT.chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .join("");
    println!("input = {}", input);

    decode_packet(input.as_str(), 0);

    unsafe {
        println!("{}", PACKET_VERSIONS);
        // 821
    }
}

static mut PACKET_VERSIONS: i32 = 0;

fn decode_packet(input: &str, start: usize) -> usize {
    let offset = start;

    let type_id = get_bits(input, offset + 3, 3);

    let packet_version = get_bits(input, offset, 3);
    let packet_version = i32::from_str_radix(packet_version, 2).unwrap();
    println!("{}", packet_version);
    unsafe {
        PACKET_VERSIONS += packet_version;
    }

    // println!("first three bits: {}", packet_version);
    // println!("second three bits: {}", type_id);

    // decode data
    match type_id {
        "100" => read_literal_value(input, offset + 6),
        _ => read_operator_packet(input, offset + 6)
    }
}

fn read_operator_packet(input: &str, start: usize) -> usize {
    let length_type_id = get_bits(input, start, 1);

    match length_type_id {
        "1" => {
            let number_of_sub_packets = get_bits(input, start + 1, 11);
            let mut offset = start + 12;

            println!("number of sub packets: {}", number_of_sub_packets);
            for _ in 0..i32::from_str_radix(number_of_sub_packets, 2).unwrap() {
                offset = decode_packet(input, offset);
            }

            offset
        }
        "0" => {
            let total_length_in_bits = usize::from_str_radix(get_bits(input, start + 1, 15), 2).unwrap();
            let mut offset = start + 16;

            println!("offset: {} -- total length in bits: {} -- start: {}", (offset), total_length_in_bits, start);
            while offset != (total_length_in_bits + start + 16) {
                // println!("--> offset: {}", offset);
                offset = decode_packet(input, offset);
            }

            offset
        }
        _ => panic!()
    }
}

fn read_literal_value(input: &str, start: usize) -> usize {
    let mut offset = start;
    let mut value = 0;
    loop {
        let four_bits_of_number = usize::from_str_radix(get_bits(input, offset + 1, 4), 2).unwrap();
        value = (value << 4) + four_bits_of_number;

        let prefix_bit = get_bits(input, offset, 1);
        offset += 5;

        if prefix_bit == "0" { break; }
    }

    println!("literal value = {} // offset = {}", value, offset);
    offset
}

fn get_bits(bits: &str, start: usize, length: usize) -> &str {
    // let mut bits_len = (bits as f64).log2().ceil() as u32;
    // if bits_len % 4 != 0 {bits_len = bits_len + (4 - (bits_len % 4)) };
    // println!("bits: {:#b} -- start: {} -- bits len: {}", bits, start, bits_len);
    // let bits = (bits << start) & (2u64.pow(bits_len) - 1);
    // println!("bits: {:#b}", bits);
    //
    // let one_bits_of_len = 2u64.pow(length) - 1;
    //
    // (bits & (one_bits_of_len << (bits_len - length))) >> (bits_len - length)
    &bits[start..start + length]
}

fn part2() {}

fn main() {
    part1();
}