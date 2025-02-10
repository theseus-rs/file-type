use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_235: FileType = FileType {
    file_format: &FileFormat {
        id: 235,
        source_type: SourceType::Linguist,
        name: "Module Management System",
        extensions: &["mmk", "mms"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
