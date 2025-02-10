use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1718066417: FileType = FileType {
    file_format: &FileFormat {
        id: 1_718_066_417,
        source_type: SourceType::Httpd,
        name: "fli",
        extensions: &["fli"],
        media_types: &["video/x-fli"],
        signatures: &[],
        related_formats: &[],
    },
};
