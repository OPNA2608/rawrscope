#![windows_subsystem = "console"]

mod args;
mod audio;
mod commands;
mod config;
mod panic;
mod render;
mod scope;
mod state;
mod ui;

fn main() {
    let matches = args::get().get_matches();

    let colors = fern::colors::ColoredLevelConfig::new();
    let level_filter = match matches.occurrences_of("VERBOSE") {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{:>6}@{}  {}",
                colors.color(record.level()),
                record.target(),
                message,
            ))
        })
        .level(level_filter)
        .chain(std::io::stdout())
        .apply()
        .expect("could not initialize logging"); // TODO dont panic?

    match matches.subcommand_name() {
        None => commands::app::run(matches.value_of("PROJECT")),
        Some("configure_audio") => commands::configure_audio::run(),
        _ => unimplemented!(),
    }
}
