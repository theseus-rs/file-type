use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3795678043: FileType = FileType {
    file_format: &FileFormat {
        id: 3_795_678_043,
        source_type: SourceType::Httpd,
        name: "ms wm",
        extensions: &["wm"],
        media_types: &["video/x-ms-wm"],
        signatures: &[],
        related_formats: &[],
    },
};
