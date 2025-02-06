use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4185802974: FileFormat = FileFormat {
    id: 4_185_802_974,
    source_type: SourceType::Iana,
    name: "alto-propmap+json",
    extensions: &[],
    media_types: &["application/alto-propmap+json"],
    signatures: &[],
    related_formats: &[],
};
