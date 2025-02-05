use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_29: FileFormat = FileFormat {
    id: 29,
    source_type: SourceType::Linguist,
    name: "Batchfile",
    extensions: &["bat", "cmd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
