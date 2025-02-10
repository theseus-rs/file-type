use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_632170566: FileType = FileType {
    file_format: &FileFormat {
        id: 632_170_566,
        source_type: SourceType::Httpd,
        name: "handheld entertainment xml",
        extensions: &["zmm"],
        media_types: &["application/vnd.handheld-entertainment+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
