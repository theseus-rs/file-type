use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_421: FileType = FileType {
    file_format: &FileFormat {
        id: 421,
        source_type: SourceType::Linguist,
        name: "xBase",
        extensions: &["ch", "prg", "prw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
