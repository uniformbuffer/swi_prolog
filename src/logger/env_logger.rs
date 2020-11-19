use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init_env_logger() {
    let mut builder = Builder::from_default_env();
    builder.format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args())).filter(None, LevelFilter::Trace).init();
}
