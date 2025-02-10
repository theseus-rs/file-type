use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2892537491: FileType = FileType {
    file_format: &FileFormat {
        id: 2_892_537_491,
        source_type: SourceType::Httpd,
        name: "smaf",
        extensions: &["mmf"],
        media_types: &["application/vnd.smaf"],
        signatures: &[],
        related_formats: &[],
    },
};
