use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_95: FileFormat = FileFormat {
    id: 95,
    source_type: SourceType::Linguist,
    name: "EJS",
    extensions: &["ect", "ejs", "ejs.t", "jst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
