use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2126647565: FileType = FileType {
    file_format: &FileFormat {
        id: 2_126_647_565,
        source_type: SourceType::Httpd,
        name: "xiff",
        extensions: &["xif"],
        media_types: &["image/vnd.xiff"],
        signatures: &[],
        related_formats: &[],
    },
};
