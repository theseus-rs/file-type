use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3752520368: FileFormat = FileFormat {
    id: 3_752_520_368,
    source_type: SourceType::Iana,
    name: "pkcs7-signature",
    extensions: &[],
    media_types: &["application/pkcs7-signature"],
    internal_signatures: &[],
    related_formats: &[],
};
