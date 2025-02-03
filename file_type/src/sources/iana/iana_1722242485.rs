use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1722242485: FileFormat = FileFormat {
    id: 1_722_242_485,
    source_type: SourceType::Iana,
    name: "vnd.google-earth.kmz",
    extensions: &[],
    media_types: &["application/vnd.google-earth.kmz"],
    internal_signatures: &[],
    related_formats: &[],
};
