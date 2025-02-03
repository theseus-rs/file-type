use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_914146169: FileFormat = FileFormat {
    id: 914_146_169,
    source_type: SourceType::Httpd,
    name: "seemail",
    extensions: &["see"],
    media_types: &["application/vnd.seemail"],
    internal_signatures: &[],
    related_formats: &[],
};
