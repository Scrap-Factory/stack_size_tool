use std::ops::Index;

use serde::Deserialize;

use crate::utill::FromFile;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Db {
    pub shape_set_list: Option<DbType>,
    pub harvestable_set_list: Option<DbType>,
    pub tool_set_list: Option<DbType>,
    pub scriptable_object_set_list: Option<DbType>,
    pub character_set_list: Option<DbType>,
}

impl FromFile<Db> for Db {}

impl Index<&str> for Db {
    type Output = Option<DbType>;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "shape_set_list" => &self.shape_set_list,
            "scriptable_object_set_list" => &self.scriptable_object_set_list,
            "harvestable_set_list" => &self.harvestable_set_list,
            "tool_set_list" => &self.tool_set_list,
            "character_set_list" => &self.character_set_list,
            _ => &self.shape_set_list,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
///Gotta love sm being consitent
pub enum DbType {
    Weird(Vec<Weird>),
    Vec(Vec<String>),
}

impl DbType {
    pub fn get_vec(&self, set_entry: Option<&str>) -> Option<Vec<String>> {
        match &self {
            DbType::Weird(weird) => {
                let mut vec = Vec::new();
                for weird in weird {
                    if let Some(set_entry) = set_entry {
                        if let Some(set_string) = &weird[set_entry] {
                            vec.push(set_string.clone());
                        }
                    }
                }

                Some(vec)
            }
            DbType::Vec(vec) => Some(vec.clone()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weird {
    pub scriptable_object_set: Option<String>,
    #[serde(alias = "name")]
    pub harvestable_object_set: Option<String>,
}

impl Index<&str> for Weird {
    type Output = Option<String>;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "scriptable_object_set" => &self.scriptable_object_set,
            "harvestable_object_set" => &self.harvestable_object_set,
            _ => &self.harvestable_object_set,
        }
    }
}
