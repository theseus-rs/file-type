use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1841200561: FileFormat = FileFormat {
    id: 1_841_200_561,
    source_type: SourceType::Iana,
    name: "sru+xml",
    extensions: &[],
    media_types: &["application/sru+xml"],
    signatures: &[],
    related_formats: &[],
};
