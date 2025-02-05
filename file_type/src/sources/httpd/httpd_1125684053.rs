use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1125684053: FileFormat = FileFormat {
    id: 1_125_684_053,
    source_type: SourceType::Httpd,
    name: "resource lists diff xml",
    extensions: &["rld"],
    media_types: &["application/resource-lists-diff+xml"],
    signatures: &[],
    related_formats: &[],
};
