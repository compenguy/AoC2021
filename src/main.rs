use anyhow::Result;

use aoc2021::{day1, day10, day2, day3, day4, day5, day6, day7, day8, day9};

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

    Ok(())
}
