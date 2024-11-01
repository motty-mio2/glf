extern crate skim;
use skim::prelude::*;
use std::io::Cursor;

mod config;
pub fn main() {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let input = "aaaaa\nbbbb\nccc".to_string();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    // println!("{}", config::get_default_host());
    // print!("{:?}", config::get_hosts());
}
