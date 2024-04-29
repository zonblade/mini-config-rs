mod config;
mod other_sample;

fn main() {
    // initialize!
    config::init();

    // call function from other file
    other_sample::sample::test_get_data();

    for c in 0..32_000_000 {
        let _ = config::SampleConfig::DatabaseName.get_str();
        // print reset line
        print!("\r{}",c);
    }
    println!("\nDone!");
}
