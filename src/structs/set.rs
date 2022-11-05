use std::ops::Index;

use serde::Deserialize;
use serde_json::Value;

use crate::utill::FromFile;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub part_list: Option<Vec<Value>>,
    pub block_list: Option<Vec<Value>>,
}

impl FromFile<Set> for Set {}

impl Index<&str> for Set {
    type Output = Option<Vec<Value>>;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "part_lst" => &self.part_list,
            "block_lst" => &self.block_list,
            _ => &self.part_list,
        }
    }
}
