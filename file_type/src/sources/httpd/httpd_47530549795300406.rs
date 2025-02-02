use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_47530549795300406: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "resource lists xml",
    extensions: &["rl"],
    media_types: &["application/resource-lists+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
