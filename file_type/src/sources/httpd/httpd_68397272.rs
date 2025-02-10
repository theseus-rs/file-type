use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_68397272: FileType = FileType {
    file_format: &FileFormat {
        id: 68_397_272,
        source_type: SourceType::Httpd,
        name: "fastbidsheet",
        extensions: &["fbs"],
        media_types: &["image/vnd.fastbidsheet"],
        signatures: &[],
        related_formats: &[],
    },
};
