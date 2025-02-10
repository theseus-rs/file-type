use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2063648569: FileType = FileType {
    file_format: &FileFormat {
        id: 2_063_648_569,
        source_type: SourceType::Httpd,
        name: "groove tool template",
        extensions: &["tpl"],
        media_types: &["application/vnd.groove-tool-template"],
        signatures: &[],
        related_formats: &[],
    },
};
