use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15474808736122015508: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "troff",
    extensions: &["t", "tr", "roff", "man", "me", "ms"],
    media_types: &["text/troff"],
    internal_signatures: &[],
    related_formats: &[],
};
