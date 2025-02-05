use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_355: FileFormat = FileFormat {
    id: 355,
    source_type: SourceType::Linguist,
    name: "Squirrel",
    extensions: &["nut"],
    media_types: &["text/x-c++src"],
    signatures: &[],
    related_formats: &[],
};
