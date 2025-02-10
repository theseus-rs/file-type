use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2074214691: FileType = FileType {
    file_format: &FileFormat {
        id: 2_074_214_691,
        source_type: SourceType::Httpd,
        name: "rtf",
        extensions: &["rtf"],
        media_types: &["application/rtf"],
        signatures: &[],
        related_formats: &[],
    },
};
