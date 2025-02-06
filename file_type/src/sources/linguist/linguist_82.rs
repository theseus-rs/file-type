use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_82: FileFormat = FileFormat {
    id: 82,
    source_type: SourceType::Linguist,
    name: "DIGITAL Command Language",
    extensions: &["com"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
