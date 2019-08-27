#[macro_use] extern crate lazy_static;
extern crate regex;

mod parser;
mod scopes;

use parser::parse;
use scopes::scope;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args.last().unwrap().trim();
  let tree = parse::parse(program);
  let scope = scope::create_scope(None);
  println!("{:?}", tree);
  println!("{:?}", scope);
}
