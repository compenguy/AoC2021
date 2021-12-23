use anyhow::Result;

use aoc2021::{
    day1, day10, day11, day12, day13, day14, day15, day16, day2, day3, day4, day5, day6, day7,
    day8, day9,
};

fn main() -> Result<()> {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data");

    // Day 1
    let day1_data = day1::data(&data_dir)?;
    let star1_count = day1::star1(&day1_data);
    println!("[Star 1] Count of increasing depths: {}", star1_count);
    let star2_count = day1::star2(&day1_data);
    println!(
        "[Star 2] Count of increasing depths of window size 3: {}",
        star2_count
    );

    // Day 2
    let day2_data = day2::data(&data_dir)?;
    let star1_count = day2::star1(&day2_data);
    println!("[Star 1] Travel distance: {}", star1_count);
    let star2_count = day2::star2(&day2_data);
    println!("[Star 2] Travel distance: {}", star2_count);

    // Day 3
    let mut day3_data = day3::data(&data_dir)?;
    let star1_count = day3::star1(&day3_data);
    println!("[Star 1] Power consumption: {}", star1_count);

    let star2_count = day3::star2(&mut day3_data);
    println!("[Star 2] Life support: {}", star2_count);

    // Day 4
    let (called, mut boards) = day4::data(&data_dir)?;
    let star1_count = day4::star1(&called, &mut boards);
    println!("[Star 1] Bingo high score: {}", star1_count);

    let star2_count = day4::star2(&called, &mut boards);
    println!("[Star 2] Bingo low score: {}", star2_count);

    // Day 5
    let ranges = day5::data(&data_dir)?;
    let star1_count = day5::star1(&ranges);
    println!(
        "[Star 1] Points with normal overlapping ranges: {}",
        star1_count
    );

    let star2_count = day5::star2(&ranges);
    println!(
        "[Star 1] Points with any overlapping ranges: {}",
        star2_count
    );

    // Day 6
    let pond = day6::data(&data_dir)?;
    let star1_count = day6::star1(&pond);
    println!("[Star 1] Fish in pond after 80 days: {}", star1_count);

    let star2_count = day6::star2(&pond);
    println!("[Star 1] Fish in pond after 256 days: {}", star2_count);

    // Day 7
    let data = day7::data(&data_dir)?;
    let star1_count = day7::star1(&data);
    println!("[Star 1] Fuel usage: {}", star1_count);

    let star2_count = day7::star2(&data);
    println!("[Star 2] Fuel usage: {}", star2_count);

    // Day 8
    let data = day8::data(&data_dir)?;
    let star1_count = day8::star1(data.as_ref());
    println!("[Star 1] Unique light patterns: {}", star1_count);

    let star2_count = day8::star2(data.as_ref());
    println!("[Star 2] Display sum: {}", star2_count);

    // Day 9
    let data = day9::data(&data_dir)?;
    let data: Vec<&[u8]> = data.iter().map(|r| r.as_slice()).collect();
    let star1_count = day9::star1(data.as_ref());
    println!("[Star 1] Low point risk: {}", star1_count);

    let star2_count = day9::star2(data.as_ref());
    println!("[Star 2] Basin size product: {}", star2_count);

    // Day 10
    let data = day10::data(&data_dir)?;
    let star1_count = day10::star1(data.as_ref());
    println!("[Star 1] Parse error score: {}", star1_count);

    let star2_count = day10::star2(data.as_ref());
    println!("[Star 2] Parse completion score: {}", star2_count);

    // Day 11
    let data = day11::data(&data_dir)?;
    let star1_count = day11::star1(data.clone());
    println!("[Star 1] Flashes after 100 steps: {}", star1_count);

    let star2_count = day11::star2(data);
    println!("[Star 2] Synchronization time: {}", star2_count);

    // Day 12
    let data = day12::data(&data_dir)?;
    let star1_count = day12::star1(&data);
    println!("[Star 1] Maze paths: {}", star1_count);

    let star2_count = day12::star2(&data);
    println!("[Star 2] Maze paths with one revisit: {}", star2_count);

    // Day 13
    let data = day13::data(&data_dir);
    let (points, folds) = day13::parse(data);
    let star1_count = day13::star1((points.clone(), &folds));
    println!("[Star 1] Points after one fold: {}", star1_count);

    let star2_count = day13::star2((points, &folds));
    println!("[Star 2] Points after all folds: {}", star2_count);

    // Day 14
    let data = day14::data(&data_dir);
    let (polymer, insertions) = day14::parse(data);
    let star1_count = day14::star1(&polymer, &insertions);
    println!("[Star 1] Polymer score after 10 steps: {}", star1_count);

    let star2_count = day14::star2(&polymer, &insertions);
    println!("[Star 2] Polymer score after 40 steps: {}", star2_count);

    // Day 15
    let data = day15::data(&data_dir);
    let maze = day15::parse(data);
    let star1_count = day15::star1(&maze);
    println!("[Star 1] Maze risk score: {}", star1_count);

    let star2_count = day15::star2(&maze);
    println!("[Star 2] Maze risk score: {}", star2_count);

    // Day 16
    let data = day16::data(&data_dir);
    let message = day16::parse(data);
    let star1_count = day16::star1(&message);
    println!("[Star 1] Version sum: {}", star1_count);

    let star2_count = day16::star2(&message);
    println!("[Star 2] Calculation: {}", star2_count);
    Ok(())
}
