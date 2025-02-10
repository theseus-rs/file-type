use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2596484608: FileType = FileType {
    file_format: &FileFormat {
        id: 2_596_484_608,
        source_type: SourceType::Httpd,
        name: "3ds",
        extensions: &["3ds"],
        media_types: &["image/x-3ds"],
        signatures: &[],
        related_formats: &[],
    },
};
