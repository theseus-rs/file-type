use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4077927361: FileFormat = FileFormat {
    id: 4_077_927_361,
    source_type: SourceType::Iana,
    name: "ace-trl+cbor",
    extensions: &[],
    media_types: &["application/ace-trl+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
