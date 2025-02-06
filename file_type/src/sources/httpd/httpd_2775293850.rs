use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2775293850: FileFormat = FileFormat {
    id: 2_775_293_850,
    source_type: SourceType::Httpd,
    name: "troff",
    extensions: &["t", "tr", "roff", "man", "me", "ms"],
    media_types: &["text/troff"],
    signatures: &[],
    related_formats: &[],
};
