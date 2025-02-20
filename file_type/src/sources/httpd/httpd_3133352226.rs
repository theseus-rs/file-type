use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3133352226: FileType = FileType {
    file_format: &FileFormat {
        id: 3_133_352_226,
        source_type: SourceType::Httpd,
        name: "jpeg",
        extensions: &["jpeg", "jpg", "jpe"],
        media_types: &["image/jpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
