use std::ops::ControlFlow;

#[derive(Debug, Copy, Clone)]
enum Sign {
    Positive,
    Negative,
}

fn main() {
    let data = include_str!("../../resources/day_2/puzzle_2.txt");

    // Parse reports
    let reports = data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let valid_reports = reports
        .into_iter()
        .filter_map(|original_report| {
            // Problem dampener
            let report_variants = std::iter::repeat(original_report.clone())
                .zip(0..original_report.len())
                .map(|(mut report, i)| {
                    report.remove(i);
                    report
                });

            report_variants
                .chain(std::iter::once(original_report))
                .find_map(|report| {
                    let mut sign: Option<Sign> = None;

                    // Iterate over values pair-wise
                    let result = report
                        .windows(2)
                        .map(|pair| pair[0] - pair[1])
                        .try_for_each(|diff| {
                            let abs_diff = diff.abs();
                            if abs_diff > 3 || abs_diff < 1 {
                                return ControlFlow::Break(());
                            }

                            match (diff, sign) {
                                (d, Some(Sign::Positive) | None) if d.is_positive() => {
                                    sign = Some(Sign::Positive);
                                    ControlFlow::Continue(())
                                }
                                (d, Some(Sign::Negative) | None) if d.is_negative() => {
                                    sign = Some(Sign::Negative);
                                    ControlFlow::Continue(())
                                }
                                _ => ControlFlow::Break(()),
                            }
                        });

                    match result {
                        ControlFlow::Break(_) => None,
                        ControlFlow::Continue(_) => Some(()),
                    }
                })
        })
        .count();

    dbg!(valid_reports);
}
