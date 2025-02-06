use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_175: FileFormat = FileFormat {
    id: 175,
    source_type: SourceType::Linguist,
    name: "JSON5",
    extensions: &["json5"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
