mod config;
mod other_sample;

fn main() {
    // initialize!
    config::init();

    // call function from other file
    other_sample::sample::test_get_data();
}
