use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_15: FileFormat = FileFormat {
    id: 15,
    source_type: SourceType::Linguist,
    name: "Ant Build System",
    extensions: &[],
    media_types: &["application/xml"],
    signatures: &[],
    related_formats: &[],
};
