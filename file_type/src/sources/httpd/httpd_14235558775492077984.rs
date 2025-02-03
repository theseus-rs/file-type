use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14235558775492077984: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "plain",
    extensions: &["txt", "text", "conf", "def", "list", "log", "in"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
