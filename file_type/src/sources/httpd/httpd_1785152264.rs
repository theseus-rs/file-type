use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1785152264: FileType = FileType {
    file_format: &FileFormat {
        id: 1_785_152_264,
        source_type: SourceType::Httpd,
        name: "ctc posml",
        extensions: &["pml"],
        media_types: &["application/vnd.ctc-posml"],
        signatures: &[],
        related_formats: &[],
    },
};
