use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_434037104: FileFormat = FileFormat {
    id: 434_037_104,
    source_type: SourceType::Iana,
    name: "at+jwt",
    extensions: &[],
    media_types: &["application/at+jwt"],
    internal_signatures: &[],
    related_formats: &[],
};
