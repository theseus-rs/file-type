use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3145740509: FileFormat = FileFormat {
    id: 3_145_740_509,
    source_type: SourceType::Iana,
    name: "vnd.umajin",
    extensions: &[],
    media_types: &["application/vnd.umajin"],
    signatures: &[],
    related_formats: &[],
};
