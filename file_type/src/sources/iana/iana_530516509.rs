use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_530516509: FileFormat = FileFormat {
    id: 530_516_509,
    source_type: SourceType::Iana,
    name: "reputon+json",
    extensions: &[],
    media_types: &["application/reputon+json"],
    internal_signatures: &[],
    related_formats: &[],
};
