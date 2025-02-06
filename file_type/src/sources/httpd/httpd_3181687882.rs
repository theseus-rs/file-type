use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3181687882: FileFormat = FileFormat {
    id: 3_181_687_882,
    source_type: SourceType::Httpd,
    name: "groove tool message",
    extensions: &["gtm"],
    media_types: &["application/vnd.groove-tool-message"],
    signatures: &[],
    related_formats: &[],
};
