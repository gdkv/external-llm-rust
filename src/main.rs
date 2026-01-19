mod content;
use content::config::Config;
use std::io::Error;

fn main() {
    let config: Result<Config, Error> = Config::load();
    let mut input: String = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Ошибка при чтении ввода.");


    println!(
        "{} {}",
        config.unwrap().openai_api_key,
        input
    );
}
