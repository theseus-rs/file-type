use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_29707939: FileFormat = FileFormat {
    id: 29_707_939,
    source_type: SourceType::Iana,
    name: "woff",
    extensions: &[],
    media_types: &["font/woff"],
    signatures: &[],
    related_formats: &[],
};
