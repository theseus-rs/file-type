use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3795678043: FileFormat = FileFormat {
    id: 3_795_678_043,
    source_type: SourceType::Httpd,
    name: "ms wm",
    extensions: &["wm"],
    media_types: &["video/x-ms-wm"],
    internal_signatures: &[],
    related_formats: &[],
};
