use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub fn day1_1(input: &str) {
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

pub fn day1_2(input: &str) {
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
