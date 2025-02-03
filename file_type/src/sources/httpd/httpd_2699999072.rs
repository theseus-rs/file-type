use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2699999072: FileFormat = FileFormat {
    id: 2_699_999_072,
    source_type: SourceType::Httpd,
    name: "chat",
    extensions: &["chat"],
    media_types: &["application/x-chat"],
    internal_signatures: &[],
    related_formats: &[],
};
