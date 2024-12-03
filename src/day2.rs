use std::str::FromStr;

#[derive(PartialEq)]
enum V {
    Start,
    Value(i32),
    Err,
}

#[derive(PartialEq, Clone)]
enum D {
    Up,
    Down,
}

impl V {
    fn option(&self) -> Option<()> {
        match *self {
            V::Value(_) => Some(()),
            _ => None,
        }
    }
}

pub fn day2_1(input: &str) {
    let result = input
        .split("\n")
        .filter_map(|i| {
            i.split(" ")
                .filter_map(|i| i32::from_str(i).ok())
                .fold((V::Start, None), |(a, d), b| match a {
                    V::Start => (V::Value(b), None),
                    V::Value(a) => {
                        let nd = match a - b {
                            i if i > 0 => D::Up,
                            i if i < 0 => D::Down,
                            _ => return (V::Err, None),
                        };
                        let d = d.unwrap_or(nd.clone());

                        if nd != d {
                            return (V::Err, None);
                        };

                        if 1 <= (a - b).abs() && (a - b).abs() <= 3 {
                            (V::Value(b), Some(nd))
                        } else {
                            (V::Err, None)
                        }
                    }
                    V::Err => (V::Err, None),
                })
                .0
                .option()
        })
        .count();

    println!("{:?}", result);
}
