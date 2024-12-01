use itertools::Itertools;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let day = std::env::args()
        .skip(1)
        .take(1)
        .fold(String::new(), |_, val| val);

    let input_path = format!("inputs/{}.txt", day);
    let mut input_file = std::fs::File::open(input_path.clone())
        .expect(&format!("Could't open {} file", input_path));
    let mut input = String::new();
    input_file
        .read_to_string(&mut input)
        .expect("Couldn't read a file");
    day1_1(&input);
}

fn day1_1(input: &str) {
    let (mut a, mut b): (Vec<_>, Vec<_>) = input
        .split("\n")
        .filter_map(|i| {
            i.split("   ")
                .filter_map(|i| i32::from_str(i).ok())
                .next_tuple()
        })
        .unzip();
    a.sort();
    b.sort();

    let result: i32 = a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum();

    println!("{:?}", result);
}
