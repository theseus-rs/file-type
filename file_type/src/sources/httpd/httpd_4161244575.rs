use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4161244575: FileType = FileType {
    file_format: &FileFormat {
        id: 4_161_244_575,
        source_type: SourceType::Httpd,
        name: "ms pki stl",
        extensions: &["stl"],
        media_types: &["application/vnd.ms-pki.stl"],
        signatures: &[],
        related_formats: &[],
    },
};
