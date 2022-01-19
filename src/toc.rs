use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::safepath::SafePathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct TOC {
   pub (in crate) _table: HashMap<SafePathBuf, u128>,
}

impl TOC {
   pub fn new() -> Self {
      TOC {
         _table: HashMap::new(),
      }
   }
}