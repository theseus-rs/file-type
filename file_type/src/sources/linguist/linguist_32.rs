use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_32: FileFormat = FileFormat {
    id: 32,
    source_type: SourceType::Linguist,
    name: "BitBake",
    extensions: &["bb", "bbappend", "bbclass", "inc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
