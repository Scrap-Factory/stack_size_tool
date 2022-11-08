use std::ops::Index;

use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utill::FromFile;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_list: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl Set {
    pub fn set_entry(&mut self, entry: &str, vec: Vec<Value>) {
        match entry {
            "part_list" => {
                self.part_list = Some(vec);
            }
            "block_list" => {
                self.block_list = Some(vec);
            }
            _ => {
                error!("Entry {entry} doesn't exist");
            }
        }
    }
}
