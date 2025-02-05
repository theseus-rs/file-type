use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_217: FileFormat = FileFormat {
    id: 217,
    source_type: SourceType::Linguist,
    name: "MAXScript",
    extensions: &["mcr", "ms"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
