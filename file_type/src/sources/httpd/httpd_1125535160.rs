use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1125535160: FileFormat = FileFormat {
    id: 1_125_535_160,
    source_type: SourceType::Httpd,
    name: "xbitmap",
    extensions: &["xbm"],
    media_types: &["image/x-xbitmap"],
    signatures: &[],
    related_formats: &[],
};
