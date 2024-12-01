fn main() {
    let data = include_str!("../resources/day_1/puzzle_1.txt");

    let (mut col1, mut col2): (Vec<_>, Vec<_>) = data
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<_>>())
        .map(|pair| {
            (
                pair[0].parse::<u64>().unwrap(),
                pair[1].parse::<u64>().unwrap(),
            )
        })
        .unzip();

    col1.sort();
    col2.sort();

    let diffs = col1
        .iter()
        .zip(col2)
        .fold(0, |acc, (a, b)| acc + a.abs_diff(b));

    dbg!(diffs);
}
