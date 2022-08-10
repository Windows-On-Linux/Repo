use std::env;
use std::fs;

fn main() {

    let mut parser = ArgumentParser::new(Some("WRU - Wineget Repo Updater"));

    let contents = fs::read_to_string("./repository.json")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
