use fern::colors::{Color, ColoredLevelConfig};
use std::io;

pub fn init(verbosity: u64) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => {
            // Let's say we depend on something which whose "info" level messages are too
            // verbose to include in end-user output. If we don't need them,
            // let's not include them.
            base_config
                .level(log::LevelFilter::Info)
                .level_for("overly-verbose-target", log::LevelFilter::Warn)
        }

        1 => base_config.level(log::LevelFilter::Debug),
        69 => {
            println!("your very immature");
            std::process::exit(-1)
        }
        420 => {
            println!("hell yeah my guy");
            std::process::exit(-1)
        }
        _2_or_more => base_config.level(log::LevelFilter::Trace),
    };

    // Separate file config so we can include year, month and day in file logs
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] ({:5}) {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S.%f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file("catnetd.log")?);

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            let colors = ColoredLevelConfig::new()
                .trace(Color::BrightBlack)
                .debug(Color::BrightCyan)
                .info(Color::BrightGreen)
                .warn(Color::BrightYellow)
                .error(Color::BrightRed);

            // special format for debug messages coming from our own crate.
            out.finish(format_args!(
                "{} [{}] ({:5}) {} {}\x1b[0m",
                format_args!("\x1b[{}m", colors.get_color(&record.level()).to_fg_str()),
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()?;

    Ok(())
}
