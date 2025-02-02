use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17814697578080827582: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sru xml",
    extensions: &["sru"],
    media_types: &["application/sru+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
