extern crate regex;
mod parser;
use parser::parse;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args.last().unwrap();
  let tree = parse::parse_expression(program);
  println!("{:?}", tree);
}
