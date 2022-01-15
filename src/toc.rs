use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TOC {
   pub _table: HashMap<PathBuf, u128>,
}

impl TOC {
   pub fn new() -> Self {
      TOC {
         _table: HashMap::new(),
      }
   }
}