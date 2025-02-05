use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4211020277: FileFormat = FileFormat {
    id: 4_211_020_277,
    source_type: SourceType::Httpd,
    name: "ms wma",
    extensions: &["wma"],
    media_types: &["audio/x-ms-wma"],
    signatures: &[],
    related_formats: &[],
};
