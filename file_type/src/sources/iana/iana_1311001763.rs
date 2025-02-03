use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1311001763: FileFormat = FileFormat {
    id: 1_311_001_763,
    source_type: SourceType::Iana,
    name: "ohttp-res",
    extensions: &[],
    media_types: &["message/ohttp-res"],
    internal_signatures: &[],
    related_formats: &[],
};
