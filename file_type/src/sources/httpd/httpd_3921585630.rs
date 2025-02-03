use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3921585630: FileFormat = FileFormat {
    id: 3_921_585_630,
    source_type: SourceType::Httpd,
    name: "mpegurl",
    extensions: &["mxu", "m4u"],
    media_types: &["video/vnd.mpegurl"],
    internal_signatures: &[],
    related_formats: &[],
};
