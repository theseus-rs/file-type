use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_200188636: FileFormat = FileFormat {
    id: 200_188_636,
    source_type: SourceType::Iana,
    name: "vnd.document+json",
    extensions: &[],
    media_types: &["application/vnd.document+json"],
    internal_signatures: &[],
    related_formats: &[],
};
