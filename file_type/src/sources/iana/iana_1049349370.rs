use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1049349370: FileFormat = FileFormat {
    id: 1_049_349_370,
    source_type: SourceType::Iana,
    name: "atomsvc+xml",
    extensions: &[],
    media_types: &["application/atomsvc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
