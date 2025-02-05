use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_709074434: FileFormat = FileFormat {
    id: 709_074_434,
    source_type: SourceType::Iana,
    name: "heic",
    extensions: &[],
    media_types: &["image/heic"],
    signatures: &[],
    related_formats: &[],
};
