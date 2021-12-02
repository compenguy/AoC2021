use anyhow::Result;

mod day1;
mod day2;

fn main() -> Result<()> {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
    let day1_data = day1::data(&data_dir)?;
    let star1_count = day1::star1(&day1_data);
    println!("[Star 1] Count of increasing depths: {}", star1_count);
    let star2_count = day1::star2(&day1_data);
    println!(
        "[Star 2] Count of increasing depths of window size 3: {}",
        star2_count
    );
    let day2_data = day2::data(&data_dir)?;
    let star1_count = day2::star1(&day2_data);
    println!("[Star 1] Travel distance: {}", star1_count);
    let star2_count = day2::star2(&day2_data);
    println!("[Star 2] Travel distance: {}", star2_count);

    Ok(())
}
