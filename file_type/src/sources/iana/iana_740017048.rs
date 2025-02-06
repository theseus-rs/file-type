use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_740017048: FileFormat = FileFormat {
    id: 740_017_048,
    source_type: SourceType::Iana,
    name: "node",
    extensions: &[],
    media_types: &["application/node"],
    signatures: &[],
    related_formats: &[],
};
