use std::io::Read;

mod day2;

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
    day2::day2_1(&input);
}
