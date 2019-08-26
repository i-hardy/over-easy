pub mod parse {
  use regex::Regex;

  #[derive(Debug)]
  pub struct AST {
    egg_type: String,
    value: u32,
  }

  pub fn parse_expression<'a>(program: &'a str) -> AST {
    let re = Regex::new(r"^\d+\b").unwrap();
    if re.is_match(program) {
      let match_value = re.find(program).unwrap().as_str();
      let as_int = u32::from_str_radix(match_value, 10).unwrap();
      return AST { egg_type: String::from("value"), value: as_int }
    }
    return AST { egg_type: String::from("value"), value: 1 }
  }
}