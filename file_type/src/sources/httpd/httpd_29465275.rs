use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_29465275: FileFormat = FileFormat {
    id: 29_465_275,
    source_type: SourceType::Httpd,
    name: "trueapp",
    extensions: &["tra"],
    media_types: &["application/vnd.trueapp"],
    internal_signatures: &[],
    related_formats: &[],
};
