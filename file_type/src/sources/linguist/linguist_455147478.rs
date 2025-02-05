use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_455147478: FileFormat = FileFormat {
    id: 455_147_478,
    source_type: SourceType::Linguist,
    name: "Lean 4",
    extensions: &["lean"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
