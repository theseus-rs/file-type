use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_427393356: FileType = FileType {
    file_format: &FileFormat {
        id: 427_393_356,
        source_type: SourceType::Httpd,
        name: "unity",
        extensions: &["unityweb"],
        media_types: &["application/vnd.unity"],
        signatures: &[],
        related_formats: &[],
    },
};
