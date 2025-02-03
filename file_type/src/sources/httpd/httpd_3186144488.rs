use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3186144488: FileFormat = FileFormat {
    id: 3_186_144_488,
    source_type: SourceType::Httpd,
    name: "plain",
    extensions: &["txt", "text", "conf", "def", "list", "log", "in"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
