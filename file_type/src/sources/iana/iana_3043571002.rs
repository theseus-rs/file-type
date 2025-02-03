use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3043571002: FileFormat = FileFormat {
    id: 3_043_571_002,
    source_type: SourceType::Iana,
    name: "coap-group+json",
    extensions: &[],
    media_types: &["application/coap-group+json"],
    internal_signatures: &[],
    related_formats: &[],
};
