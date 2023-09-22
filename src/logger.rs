use log::{Level, Log, RecordBuilder};

use simplelog::{
  ConfigBuilder, TermLogger,
  LevelFilter, LevelPadding,
  TerminalMode, ColorChoice
};

pub fn setup_log(verbosity: u8) -> Result<(), eyre::Error> {
  let log_level: LevelFilter = match verbosity {
    0 => LevelFilter::Info,
    1 => LevelFilter::Debug,
    2 => LevelFilter::Trace,
    _ => eyre::bail!("There's only so much I can tell."),
  };
  let mut log_config: ConfigBuilder = ConfigBuilder::new();
  log_config
    .set_level_padding(LevelPadding::Right)
    .set_time_format_rfc3339()
  ;

  let prelogger = TermLogger::new(
    log_level.clone(),
    log_config.clone().build(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  );

  if log_config.set_time_offset_to_local().is_err() {
    let record_args = format_args!("Unable to inspect local timezone offset");

    let record = RecordBuilder::new()
      .args(record_args)
      .level(Level::Warn)
      .target(module_path!())
      .module_path_static(Some(module_path!()))
      .file_static(Some(file!()))
      .line(None)
      .build()
    ;

    prelogger.log(&record);
  }

  TermLogger::init(
    log_level,
    log_config.build(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  )?;

  Ok(())
}
