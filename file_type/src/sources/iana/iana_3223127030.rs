use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3223127030: FileFormat = FileFormat {
    id: 3_223_127_030,
    source_type: SourceType::Iana,
    name: "mbms-msk+xml",
    extensions: &[],
    media_types: &["application/mbms-msk+xml"],
    signatures: &[],
    related_formats: &[],
};
