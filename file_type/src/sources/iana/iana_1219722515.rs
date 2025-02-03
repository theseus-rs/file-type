use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1219722515: FileFormat = FileFormat {
    id: 1_219_722_515,
    source_type: SourceType::Iana,
    name: "xcap-ns+xml",
    extensions: &[],
    media_types: &["application/xcap-ns+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
