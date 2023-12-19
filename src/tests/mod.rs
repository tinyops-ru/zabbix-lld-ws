use fake::{Fake, Faker};
use log::LevelFilter;

pub mod integration;

pub fn init_logging() {
    let _ = env_logger::builder().filter_level(LevelFilter::Debug)
        .is_test(true).try_init();
}

pub fn get_random_string() -> String {
    Faker.fake::<String>()
}