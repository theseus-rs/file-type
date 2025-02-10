use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_412: FileType = FileType {
    file_format: &FileFormat {
        id: 412,
        source_type: SourceType::Linguist,
        name: "desktop",
        extensions: &["desktop", "desktop.in", "service"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
