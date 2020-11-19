#[cfg(feature = "console_logger")]
mod env_logger;

pub fn init_logger() {

    #[cfg(feature = "console_logger")]
    env_logger::init_env_logger();

    println!("Logger initialized");
}
