use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1125535160: FileType = FileType {
    file_format: &FileFormat {
        id: 1_125_535_160,
        source_type: SourceType::Httpd,
        name: "xbitmap",
        extensions: &["xbm"],
        media_types: &["image/x-xbitmap"],
        signatures: &[],
        related_formats: &[],
    },
};
