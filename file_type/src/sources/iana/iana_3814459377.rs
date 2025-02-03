use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3814459377: FileFormat = FileFormat {
    id: 3_814_459_377,
    source_type: SourceType::Iana,
    name: "alternative",
    extensions: &[],
    media_types: &["multipart/alternative"],
    internal_signatures: &[],
    related_formats: &[],
};
