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
    ASTApply {
      ast_type: ASTType,
      operator: Box<ASTExpression>,
      args: Vec<ASTExpression>,
    }
  }

  #[derive(Debug)]
  pub struct AST {
    expr: ASTExpression,
    rest: Option<String>
  }

  fn parse_apply<'a>(expr: ASTExpression, program: &'a str) -> AST {
    if &program.chars().next().unwrap_or(Default::default()) != &'(' {
      return AST { expr, rest: Some(program.to_string()) };
    }
    let mut to_parse: String = program[1..].to_string();
    let mut args: Vec<ASTExpression> = Vec::new();
    while &to_parse.chars().next().unwrap() != &')' {
      let arg = parse_expression(&to_parse);
      args.push(arg.expr);
      to_parse = arg.rest.unwrap().trim_start().to_string();
      let next_char = &to_parse.chars().next().unwrap();
      if next_char == &',' {
        to_parse = to_parse[1..].trim_start().to_string();
      } else if next_char != &')' {      
        panic!("Expected ',' or ')', received {}", next_char);
      }
    }
    let new_expr = ASTExpression::ASTApply { ast_type: ASTType::Apply, operator: Box::new(expr), args };
    return parse_apply(new_expr, &to_parse[1..]);
  }

  fn parse_expression<'a>(program: &'a str) -> AST {
    lazy_static! {
      static ref NUMBER: Regex = Regex::new(r"^\d+\b").unwrap();
      static ref STRING: Regex = Regex::new(r"^'([^']*)'").unwrap();
      static ref WORD: Regex = Regex::new("^[^\\s(),#\"]+").unwrap();
    }
    let to_parse = program.trim_start();
    if NUMBER.is_match(to_parse) {
      let value = NUMBER.find(to_parse).unwrap().as_str();
      let int_value = i32::from_str_radix(value, 10).unwrap();
      let expression = ASTExpression::ASTNumber { ast_type: ASTType::Value, value: int_value };
      return parse_apply(expression, &to_parse[value.len()..]);
    } else if STRING.is_match(to_parse) {
      let caps = STRING.captures(to_parse).unwrap();
      let full_str = caps.get(0).unwrap().as_str();
      let value = caps.get(caps.len() - 1).unwrap().as_str();
      let expression = ASTExpression::ASTString { ast_type: ASTType::Value, value: value.to_string() };
      return parse_apply(expression, &to_parse[full_str.len()..]);
    } else if WORD.is_match(to_parse) {
      let value = WORD.find(to_parse).unwrap().as_str();
      let expression = ASTExpression::ASTWord { ast_type: ASTType::Word, name: value.to_string() };
      return parse_apply(expression, &to_parse[value.len()..]);
    }
    panic!("Unexpected syntax: {}", program);
  }

  pub fn parse<'a>(program: &'a str) -> ASTExpression {
    let parsed = parse_expression(program);
    return parsed.expr;
  }
}