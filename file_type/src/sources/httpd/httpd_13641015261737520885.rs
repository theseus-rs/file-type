use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13641015261737520885: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "chemdraw xml",
    extensions: &["cdxml"],
    media_types: &["application/vnd.chemdraw+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
