use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3978301349: FileType = FileType {
    file_format: &FileFormat {
        id: 3_978_301_349,
        source_type: SourceType::Httpd,
        name: "dtbook xml",
        extensions: &["dtb"],
        media_types: &["application/x-dtbook+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
