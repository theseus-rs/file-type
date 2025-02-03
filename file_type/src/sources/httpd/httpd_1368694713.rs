use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1368694713: FileFormat = FileFormat {
    id: 1_368_694_713,
    source_type: SourceType::Httpd,
    name: "msword",
    extensions: &["doc", "dot"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
