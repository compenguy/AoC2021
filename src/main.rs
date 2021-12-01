use anyhow::{Context, Result};
use clap::{app_from_crate, crate_name, crate_version};
use flexi_logger::{colored_default_format, detailed_format, Logger};
use log::debug;

mod day1;

fn init_logging(matches: &clap::ArgMatches) -> Result<()> {
    // Default log-level is Off, and goes up with each debug flag
    let crate_log_level = match matches.occurrences_of("debug") {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        4 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    // Logging for dependencies is disabled, unless we're logging this program at Debug or Trace
    // levels, in which case we'll log Errors for dependencies.
    let general_log_level = match crate_log_level {
        log::LevelFilter::Trace | log::LevelFilter::Debug => log::LevelFilter::Error,
        _ => log::LevelFilter::Off,
    };
    // Everything logs at `general_log_level` (nothing or errors-only), except for code from this
    // crate which logs at `crate_log_level` (set by occurrences of `debug` flag)
    let spec = format!(
        "{}, {} = {}",
        general_log_level,
        crate_name!(),
        crate_log_level
    );
    let mut log_builder = Logger::try_with_str(&spec)?
        .format(detailed_format)
        .format_for_stderr(colored_default_format);

    if matches.is_present("debug-log") {
        let cache_dir = dirs::cache_dir()
            .expect("Unable to determine user cache dir")
            .join(crate_name!());
        let log_dir = cache_dir.join("logs");
        if !log_dir.is_dir() {
            std::fs::create_dir_all(&log_dir).with_context(|| {
                format!(
                    "Failed to create application log directory {}",
                    log_dir.display()
                )
            })?;
        }
        // debug log file
        log_builder =
            log_builder.log_to_file(flexi_logger::FileSpec::default().directory(&log_dir));
        println!("Logging debug output to {}", log_dir.display());
    }

    log_builder
        .start()
        .with_context(|| "Failed to start FlexiLogger logging backend")
        .map(|_| ())
}

fn main() -> Result<()> {
    let matches = app_from_crate!()
        .color(clap::ColorChoice::Auto)
        .arg(
            clap::Arg::new("debug")
                .short('g')
                .long("debug")
                .multiple_occurrences(true)
                .hidden(true)
                .about("Enable debug-level output"),
        )
        .arg(
            clap::Arg::new("debug-log")
                .short('l')
                .long("debug-log")
                .hidden(true)
                .about("Whether to write a debug log file."),
        )
        .get_matches();

    init_logging(&matches)?;
    debug!("{} version {}", crate_name!(), crate_version!());

    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
    let day1_data = day1::data(data_dir)?;
    let star1_count = day1::star1(&day1_data);
    println!("[Star 1] Count of increasing depths: {}", star1_count);
    let star2_count = day1::star2(&day1_data);
    println!(
        "[Star 2] Count of increasing depths of window size 3: {}",
        star2_count
    );

    Ok(())
}
