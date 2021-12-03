use inputs;


fn oxygen(s: Vec<String>) {
    let mut result: Vec<String> = s;

    for i in 0..12 {
        let g = gamma_rate(&result);
        let mcb = if g[i] > 0 { 1 } else { 0 };
        result = result.into_iter()
            .filter(|ss| ss.chars().nth(i).unwrap() == std::char::from_digit(mcb, 10).unwrap())
            .collect();

        if result.len() == 1 { break; }
    }

    println!("{:?}", result);
    // oxygen:
    // 100111001101

    // co2:
    // 011010100101

    // => 4267809

}

fn gamma_rate(s: &Vec<String>) -> [i32; 12] {
    let mut a = [0; 12];

    for x in s {
        for n in 0..x.len() {
            match x.chars().nth(n).unwrap() {
                '0' => a[n] += 1,
                '1' => a[n] -= 1,
                a => println!("{}", a)
            }
        }
    }

    a
}

fn day1(s: Vec<String>) -> i32 {
    let gamma_rate = gamma_rate(&s);
    println!("{:?}", gamma_rate);
    // 100111011000 = 2520

    // epsilon: 011000100111
    // 1575

    oxygen(s);

    0
}

fn main() {
    let inputs: Vec<String> = inputs::get_input_split(3);


    println!("{:?}", inputs);

    day1(inputs);
}
