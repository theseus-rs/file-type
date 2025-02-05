use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2132147605: FileFormat = FileFormat {
    id: 2_132_147_605,
    source_type: SourceType::Httpd,
    name: "webm",
    extensions: &["webm"],
    media_types: &["video/webm"],
    signatures: &[],
    related_formats: &[],
};
