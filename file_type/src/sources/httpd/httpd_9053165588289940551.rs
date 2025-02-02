use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9053165588289940551: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "javascript",
    extensions: &["js", "mjs"],
    media_types: &["text/javascript"],
    internal_signatures: &[],
    related_formats: &[],
};
