use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3000733719: FileFormat = FileFormat {
    id: 3_000_733_719,
    source_type: SourceType::Httpd,
    name: "rfc822",
    extensions: &["eml", "mime"],
    media_types: &["message/rfc822"],
    internal_signatures: &[],
    related_formats: &[],
};
