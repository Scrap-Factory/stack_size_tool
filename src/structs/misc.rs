#[derive(Debug, Clone, Copy)]
pub struct Folder {
    pub path: &'static str,
    pub file: &'static str,
    pub entries: [&'static str; 2],
    pub set_list: &'static str,
    pub set_entry: Option<&'static str>,
}
