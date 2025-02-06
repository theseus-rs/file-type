use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3414676129: FileFormat = FileFormat {
    id: 3_414_676_129,
    source_type: SourceType::Iana,
    name: "ocsp-request",
    extensions: &[],
    media_types: &["application/ocsp-request"],
    signatures: &[],
    related_formats: &[],
};
