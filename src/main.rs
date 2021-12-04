use anyhow::Result;

mod day1;
mod day2;
mod day3;
mod day4;

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

    Ok(())
}
