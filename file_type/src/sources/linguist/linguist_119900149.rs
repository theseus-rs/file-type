use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_119900149: FileType = FileType {
    file_format: &FileFormat {
        id: 119_900_149,
        source_type: SourceType::Linguist,
        name: "Slint",
        extensions: &["slint"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
