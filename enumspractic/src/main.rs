use semi_structured_logs::{error, info, log, warn, LogLevel};
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel{
    Info,
    Warning,
    Error,
}
pub fn log(level: LogLevel, message:&str) -> String{
        let level_str = format!("[{:?}]",level).to_uppercase();
        format!("{}: {}", level_str, message)
}
pub fn info(message:&str) -> String{
    log(LogLevel::Info, message)
}

pub fn info(message:&str) -> String{
    log(LogLevel::Warning, message)
}

pub fn info(message:&str) -> String{
    log(LogLevel::Error, message)
}

#[test]
fn emits_info() {
    assert_eq!(info("Timezone changed"), "[INFO]: Timezone changed");
}
