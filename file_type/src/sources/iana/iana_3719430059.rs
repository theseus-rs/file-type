use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3719430059: FileFormat = FileFormat {
    id: 3_719_430_059,
    source_type: SourceType::Iana,
    name: "application/resolve-response+jwt",
    extensions: &[],
    media_types: &["application/resolve-response+jwt"],
    internal_signatures: &[],
    related_formats: &[],
};
