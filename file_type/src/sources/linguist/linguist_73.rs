use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_73: FileType = FileType {
    file_format: &FileFormat {
        id: 73,
        source_type: SourceType::Linguist,
        name: "Csound",
        extensions: &["orc", "udo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
