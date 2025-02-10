use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2590679499: FileType = FileType {
    file_format: &FileFormat {
        id: 2_590_679_499,
        source_type: SourceType::Httpd,
        name: "xm",
        extensions: &["xm"],
        media_types: &["audio/xm"],
        signatures: &[],
        related_formats: &[],
    },
};
