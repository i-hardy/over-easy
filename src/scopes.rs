pub mod scope {
  use std::collections::HashMap;
  use std::fmt::Debug;
  use std::any::Any;

  #[derive(Debug)]
  #[derive(Clone)]
  pub struct Scope {
    booleans: HashMap<String, bool>,
    // functions: HashMap<String, Box<&'a fn(&'static dyn Any)>>
  }

  fn egg_print(arg: &'static dyn Any) {
    println!("{:?}", arg);
  }

  fn create_top_scope() -> Scope {
    let mut booleans = HashMap::new();
    booleans.insert("true".to_string(), true);
    booleans.insert("false".to_string(), false);
    // let mut functions = HashMap::new();
    // functions.insert("print".to_string(), Box::new(egg_print.clone()));
    return Scope { booleans };
  }

  pub fn create_scope(existing_scope: Option<Scope>) -> Scope {
    let scope = existing_scope.unwrap_or(create_top_scope()).clone();
    return scope;
  }
}