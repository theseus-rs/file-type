use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_393446519: FileType = FileType {
    file_format: &FileFormat {
        id: 393_446_519,
        source_type: SourceType::Httpd,
        name: "cdx",
        extensions: &["cdx"],
        media_types: &["chemical/x-cdx"],
        signatures: &[],
        related_formats: &[],
    },
};
