use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_13400192: FileFormat = FileFormat {
    id: 13_400_192,
    source_type: SourceType::Iana,
    name: "prs.implied-structure",
    extensions: &[],
    media_types: &["application/prs.implied-structure"],
    internal_signatures: &[],
    related_formats: &[],
};
