use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_20: FileFormat = FileFormat {
    id: 20,
    source_type: SourceType::Linguist,
    name: "Arc",
    extensions: &["arc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
