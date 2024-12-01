use itertools::Itertools;
use std::collections::HashMap;
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
    day1_2(&input);
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

fn day1_2(input: &str) {
    let (a, b): (Vec<_>, Vec<_>) = input
        .split("\n")
        .filter_map(|i| {
            i.split("   ")
                .filter_map(|i| i32::from_str(i).ok())
                .next_tuple()
        })
        .unzip();

    let mut hm: HashMap<i32, i32> = HashMap::new();

    b.iter().for_each(|i| {
        hm.entry(*i).and_modify(|i| *i += 1).or_insert(1);
    });

    let result: i32 = a.iter().map(|i| i * hm.get(i).unwrap_or(&0)).sum();

    println!("{:?}", result);
}
