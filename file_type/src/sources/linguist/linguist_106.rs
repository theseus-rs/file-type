use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_106: FileFormat = FileFormat {
    id: 106,
    source_type: SourceType::Linguist,
    name: "FLUX",
    extensions: &["flux", "fx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
