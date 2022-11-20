mod errors;
mod structs;
mod utill;

use crate::utill::FromFile;
use error_stack::{IntoReport, Result, ResultExt};
use errors::{FolderGenError, SetGenError};
use log::{error, info};
use serde_json::{Number, Value};
use std::{fs, path::Path};
use structs::{database::Db, misc::Folder, set::Set};

const FOLDERS: [Folder; 1] = [Folder {
    path: "./Objects/Database/",
    file: "shapesets.shapedb",
    entries: ["part_list", "block_list"],
    set_list: "shape_set_list",
    set_entry: None,
}];

const STACK_SZIE: f64 = u16::MAX as f64;

fn main() {
    simple_logger::SimpleLogger::new()
        .init()
        .expect("Failed to init a logger");

    for folder in FOLDERS.iter() {
        if !Path::new(&format!("{}{}", folder.path, folder.file)).exists() {
            continue;
        }

        match gen_folder(folder) {
            Ok(_) => {
                info!(
                    "Changed stacksize {}{} succesfully",
                    folder.path, folder.file
                );
            }
            Err(err) => {
                error!("Failed to generate {}{}", folder.path, folder.file);
                println!("{err:#?}")
            }
        }
    }
}

fn gen_folder(folder: &Folder) -> Result<(), FolderGenError> {
    let db = Db::from_file(&format!("{}{}", folder.path, folder.file))
        .change_context(FolderGenError)
        .attach_printable(format!("Failed to parse db {}{}", folder.path, folder.file))?;

    let data = db[folder.set_list]
        .as_ref()
        .ok_or(FolderGenError)
        .into_report()
        .attach_printable(format!(
            "Failed to get data out of database {}{}",
            folder.path, folder.file
        ))?;

    let data = data
        .get_vec(folder.set_entry)
        .ok_or(FolderGenError)
        .into_report()
        .attach_printable(format!(
            "Failed to get data out of database {}{}",
            folder.path, folder.file
        ))?;

    for path in data {
        gen_set(path.replace("$CONTENT_DATA", "."), folder)
            .change_context(FolderGenError)
            .attach_printable(format!("Failed to change stack size for the set {path}"))?;
    }

    Ok(())
}

fn gen_set(path: String, folder: &Folder) -> Result<(), SetGenError> {
    let set = Set::from_file(&path)
        .change_context(SetGenError)
        .attach_printable(format!("Failed to parse set {path}"))?;
    let mut file = Set {
        block_list: None,
        part_list: None,
    };

    for entry in folder.entries {
        if entry.is_empty() {
            continue;
        }

        let data = &set[entry];
        if let Some(vec) = data {
            let mut objects = Vec::new();

            for mut data in vec.clone() {
                data["stackSize"] = Value::Number(Number::from_f64(STACK_SZIE).unwrap());

                objects.push(data)
            }
            file.set_entry(entry, objects);
        }
    }

    let json = serde_json::to_string_pretty(&file)
        .into_report()
        .change_context(SetGenError)
        .attach_printable(format!("Failed to convert {file:#?} to json"))?;

    fs::write(&path, json)
        .into_report()
        .change_context(SetGenError)
        .attach_printable(format!("Failed to write to file {path}"))?;

    Ok(())
}
