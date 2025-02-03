use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3097317368: FileFormat = FileFormat {
    id: 3_097_317_368,
    source_type: SourceType::Iana,
    name: "vnd.cups-pdf",
    extensions: &[],
    media_types: &["application/vnd.cups-pdf"],
    internal_signatures: &[],
    related_formats: &[],
};
