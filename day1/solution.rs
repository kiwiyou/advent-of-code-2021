use std::fs;

fn main() {
    let (count, _) = fs::read_to_string("input.txt")
        .expect("cannot open file")
        .split_ascii_whitespace()
        .map(str::parse::<i32>)
        .flatten()
        .fold((0, None), |(count, previous), depth| match previous {
            None => (count, Some(depth)),
            Some(previous) if previous < depth => (count + 1, Some(depth)),
            _ => (count, Some(depth)),
        });
    println!("{}", count);

    let depths: Vec<i32> = fs::read_to_string("input.txt")
        .expect("cannot open file")
        .split_ascii_whitespace()
        .map(str::parse)
        .flatten()
        .collect();
    let (count, _) = depths
        .windows(3)
        .fold((0, None), |(count, previous), window| {
            let sum: i32 = window.iter().sum();
            match previous {
                None => (count, Some(sum)),
                Some(previous) if previous < sum => (count + 1, Some(sum)),
                _ => (count, Some(sum)),
            }
        });
    println!("{}", count);
}
