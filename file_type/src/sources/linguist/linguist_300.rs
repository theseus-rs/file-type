use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_300: FileFormat = FileFormat {
    id: 300,
    source_type: SourceType::Linguist,
    name: "Pure Data",
    extensions: &["pd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
