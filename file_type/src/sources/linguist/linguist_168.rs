use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_168: FileFormat = FileFormat {
    id: 168,
    source_type: SourceType::Linguist,
    name: "Io",
    extensions: &["io"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
