use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2944319513: FileFormat = FileFormat {
    id: 2_944_319_513,
    source_type: SourceType::Iana,
    name: "vnd.pwg-xhtml-print+xml",
    extensions: &[],
    media_types: &["application/vnd.pwg-xhtml-print+xml"],
    signatures: &[],
    related_formats: &[],
};
