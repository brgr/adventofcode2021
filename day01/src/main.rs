use inputs::INPUT_DAY_01;

fn count_increases(numbers: &Vec<u32>) -> u32 {
    let mut increased = 0;

    for i in 1..numbers.len() {
        if numbers[i] > numbers[i-1] { increased += 1 }
    }

    increased
}

fn count_increases_measurement_windows(numbers: &Vec<u32>) -> u32 {
    let mut increased = 0;

    for i in 3..numbers.len() {
        let this = numbers[i] + numbers[i-1] + numbers[i-2];
        let last = numbers[i-1] + numbers[i-2] + numbers[i-3];

        if this > last { increased += 1 }
    }

    increased
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", INPUT_DAY_01);

    let numbers: Vec<u32> = INPUT_DAY_01.split('\n')
        .flat_map(|i| i.parse::<u32>())
        .collect();

    println!("{}", count_increases(&numbers));
    println!("{}", count_increases_measurement_windows(&numbers));
}
