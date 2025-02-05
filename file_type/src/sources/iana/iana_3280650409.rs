use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3280650409: FileFormat = FileFormat {
    id: 3_280_650_409,
    source_type: SourceType::Iana,
    name: "scvp-cv-response",
    extensions: &[],
    media_types: &["application/scvp-cv-response"],
    signatures: &[],
    related_formats: &[],
};
