use config::Config;

mod config;

fn main() {
    let cfg = Config::default();
    let x = toml::to_string_pretty(&cfg).unwrap();

    println!("{x}");
}
