use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4066570451: FileFormat = FileFormat {
    id: 4_066_570_451,
    source_type: SourceType::Iana,
    name: "vnd.epson.quickanime",
    extensions: &[],
    media_types: &["application/vnd.epson.quickanime"],
    signatures: &[],
    related_formats: &[],
};
