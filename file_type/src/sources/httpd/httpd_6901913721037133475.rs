use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6901913721037133475: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "resource lists diff xml",
    extensions: &["rld"],
    media_types: &["application/resource-lists-diff+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
