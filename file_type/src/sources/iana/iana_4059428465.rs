use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4059428465: FileFormat = FileFormat {
    id: 4_059_428_465,
    source_type: SourceType::Iana,
    name: "vcard+json",
    extensions: &[],
    media_types: &["application/vcard+json"],
    signatures: &[],
    related_formats: &[],
};
