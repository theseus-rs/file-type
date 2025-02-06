use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_250: FileFormat = FileFormat {
    id: 250,
    source_type: SourceType::Linguist,
    name: "Ninja",
    extensions: &["ninja"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
