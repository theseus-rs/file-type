use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2590679499: FileFormat = FileFormat {
    id: 2_590_679_499,
    source_type: SourceType::Httpd,
    name: "xm",
    extensions: &["xm"],
    media_types: &["audio/xm"],
    signatures: &[],
    related_formats: &[],
};
