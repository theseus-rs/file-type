use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2401825161: FileType = FileType {
    file_format: &FileFormat {
        id: 2_401_825_161,
        source_type: SourceType::Httpd,
        name: "avif",
        extensions: &["avif"],
        media_types: &["image/avif"],
        signatures: &[],
        related_formats: &[],
    },
};
