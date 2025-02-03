use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3334640110: FileFormat = FileFormat {
    id: 3_334_640_110,
    source_type: SourceType::Httpd,
    name: "chemdraw xml",
    extensions: &["cdxml"],
    media_types: &["application/vnd.chemdraw+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
