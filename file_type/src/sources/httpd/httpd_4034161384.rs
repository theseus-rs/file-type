use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4034161384: FileType = FileType {
    file_format: &FileFormat {
        id: 4_034_161_384,
        source_type: SourceType::Httpd,
        name: "h261",
        extensions: &["h261"],
        media_types: &["video/h261"],
        signatures: &[],
        related_formats: &[],
    },
};
