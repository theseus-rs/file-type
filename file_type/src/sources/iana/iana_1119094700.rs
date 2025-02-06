use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1119094700: FileFormat = FileFormat {
    id: 1_119_094_700,
    source_type: SourceType::Iana,
    name: "odm+xml",
    extensions: &[],
    media_types: &["application/odm+xml"],
    signatures: &[],
    related_formats: &[],
};
