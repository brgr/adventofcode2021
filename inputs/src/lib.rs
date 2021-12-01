use std::fs;

pub const INPUT_DAY_01: &str = include_str!("../../resources/inputs/day01");


pub fn get_input_as_numbers(day: u8) -> Vec<i32> {
    get_input_split(day).iter()
        .map(|i| i.parse::<i32>().expect("Error when parsing a number in the input"))
        .collect()
}

pub fn get_input_as_numbers_unsigned(day: u8) -> Vec<u32> {
    get_input_split(day)
        .into_iter()
        .map(|i| i.parse::<u32>().expect("Error when parsing a number in the input"))
        .collect()
}

pub fn get_input_split(day: u8) -> Vec<String> {
    get_input_whole(day)
        .split('\n')
        .map(|s| String::from(s))
        .collect()
}

pub fn get_input_whole(day: u8) -> String {
    fs::read_to_string(format!("../resources/inputs/day{:0>2}", day))
        .expect("Could not find input file")
}

#[cfg(test)]
mod tests {
    use crate::get_input_as_numbers;

    #[test]
    fn it_works() {
        let vec = get_input_as_numbers(1);
        assert_eq!(2000, vec.len());
    }
}
