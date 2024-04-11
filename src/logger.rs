use log::info;
pub fn enable_logger() {
    pretty_env_logger::init_timed();
    info!("Init logger!");
}
