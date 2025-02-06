use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1048917476: FileFormat = FileFormat {
    id: 1_048_917_476,
    source_type: SourceType::Httpd,
    name: "xcap diff xml",
    extensions: &["xdf"],
    media_types: &["application/xcap-diff+xml"],
    signatures: &[],
    related_formats: &[],
};
