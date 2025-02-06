use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2846680230: FileFormat = FileFormat {
    id: 2_846_680_230,
    source_type: SourceType::Iana,
    name: "mets+xml",
    extensions: &[],
    media_types: &["application/mets+xml"],
    signatures: &[],
    related_formats: &[],
};
