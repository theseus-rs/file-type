use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3223149000: FileType = FileType {
    file_format: &FileFormat {
        id: 3_223_149_000,
        source_type: SourceType::Httpd,
        name: "tga",
        extensions: &["tga"],
        media_types: &["image/x-tga"],
        signatures: &[],
        related_formats: &[],
    },
};
