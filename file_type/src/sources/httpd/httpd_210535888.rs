use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_210535888: FileType = FileType {
    file_format: &FileFormat {
        id: 210_535_888,
        source_type: SourceType::Httpd,
        name: "ms word document macroenabled 12",
        extensions: &["docm"],
        media_types: &["application/vnd.ms-word.document.macroenabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
