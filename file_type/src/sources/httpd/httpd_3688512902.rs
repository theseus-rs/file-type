use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3688512902: FileType = FileType {
    file_format: &FileFormat {
        id: 3_688_512_902,
        source_type: SourceType::Httpd,
        name: "sh",
        extensions: &["sh"],
        media_types: &["application/x-sh"],
        signatures: &[],
        related_formats: &[],
    },
};
