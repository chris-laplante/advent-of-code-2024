use std::collections::HashMap;

fn main() {
    let data = include_str!("../resources/day_1/puzzle_1.txt");

    let (col1, mut col2): (Vec<_>, Vec<_>) = data
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|pair| {
            (
                pair[0].parse::<u64>().unwrap(),
                pair[1].parse::<u64>().unwrap(),
            )
        })
        .unzip();

    col2.sort();

    let occurrences = col2
        .into_iter()
        .fold(HashMap::<u64, usize>::new(), |mut acc, number| {
            acc.entry(number).and_modify(|c| *c += 1).or_insert(1);
            acc
        });

    let similarity_score = col1.into_iter().fold(0, |acc, number| {
        acc + match occurrences.get(&number) {
            Some(occ) => number * (*occ as u64),
            None => 0,
        }
    });

    dbg!(similarity_score);
}
