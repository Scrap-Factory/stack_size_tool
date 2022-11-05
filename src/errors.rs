use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct FromFileParseError;

impl Display for FromFileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to parse from file")
    }
}

impl Error for FromFileParseError {}

#[derive(Debug)]

pub struct FolderGenError;

impl Display for FolderGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to generate folder")
    }
}

impl Error for FolderGenError {}

#[derive(Debug)]
pub struct SetGenError;

impl Display for SetGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to generate set")
    }
}

impl Error for SetGenError {}
