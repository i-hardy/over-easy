pub mod parse {
  use regex::Regex;
  
  #[derive(Debug)]
  pub enum ASTType {
    Apply,
    Word,
    Value,
  }

  #[derive(Debug)]
  pub enum ASTExpression {
    ASTNumber {
      ast_type: ASTType,
      value: i32,
    },
    ASTString {
      ast_type: ASTType,
      value: String,
    },
    ASTWord {
      ast_type: ASTType,
      name: String,
    },
  }

  #[derive(Debug)]
  pub struct AST {
    expr: ASTExpression,
    rest: Option<String>
  }

  pub fn parse_apply<'a>(expr: ASTExpression, program: &'a str) -> AST {
    if &program.chars().next().unwrap() != &'(' {
      return AST { expr, rest: Some(program.to_string()) };
    }
    return AST { expr, rest: Some(program.to_string()) };
  }

  pub fn parse_expression<'a>(program: &'a str) -> AST {
    let num_regex = Regex::new(r"^\d+\b").unwrap();
    let str_regex = Regex::new(r"^'([^']*)'").unwrap();
    let word_regex = Regex::new("^[^\\s(),#\"]+").unwrap();
    if num_regex.is_match(program) {
      let value = num_regex.find(program).unwrap().as_str();
      let int_value = i32::from_str_radix(value, 10).unwrap();
      let expression = ASTExpression::ASTNumber { ast_type: ASTType::Value, value: int_value };
      return parse_apply(expression, &program[value.len()..]);
    } else if str_regex.is_match(program) {
      let caps = str_regex.captures(program).unwrap();
      let value = caps.get(caps.len() - 1).unwrap().as_str();
      let expression = ASTExpression::ASTString { ast_type: ASTType::Value, value: value.to_string() };
      return parse_apply(expression, &program[value.len()..]);

    } else if word_regex.is_match(program) {
      let value = word_regex.find(program).unwrap().as_str();
      let expression = ASTExpression::ASTWord { ast_type: ASTType::Word, name: value.to_string() };
      return parse_apply(expression, &program[value.len()..]);
    }
    panic!("Unexpected syntax: {}", program);
  }
}