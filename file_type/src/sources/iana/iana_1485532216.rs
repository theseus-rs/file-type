use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1485532216: FileFormat = FileFormat {
    id: 1_485_532_216,
    source_type: SourceType::Iana,
    name: "prs.cww",
    extensions: &[],
    media_types: &["application/prs.cww"],
    internal_signatures: &[],
    related_formats: &[],
};
