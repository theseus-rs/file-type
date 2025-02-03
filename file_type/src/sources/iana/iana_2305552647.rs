use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2305552647: FileFormat = FileFormat {
    id: 2_305_552_647,
    source_type: SourceType::Iana,
    name: "http",
    extensions: &[],
    media_types: &["message/http"],
    internal_signatures: &[],
    related_formats: &[],
};
