use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3452622185: FileFormat = FileFormat {
    id: 3_452_622_185,
    source_type: SourceType::Iana,
    name: "example",
    extensions: &[],
    media_types: &["video/example"],
    internal_signatures: &[],
    related_formats: &[],
};
