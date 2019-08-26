#[macro_use] extern crate lazy_static;
extern crate regex;
mod parser;
use parser::parse;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args.last().unwrap().trim();
  let tree = parse::parse(program);
  println!("{:?}", tree);
}
