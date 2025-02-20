use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_820780064: FileType = FileType {
    file_format: &FileFormat {
        id: 820_780_064,
        source_type: SourceType::Httpd,
        name: "kenameaapp",
        extensions: &["htke"],
        media_types: &["application/vnd.kenameaapp"],
        signatures: &[],
        related_formats: &[],
    },
};
