use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_805122868: FileFormat = FileFormat {
    id: 805_122_868,
    source_type: SourceType::Linguist,
    name: "YARA",
    extensions: &["yar", "yara"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
