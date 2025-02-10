use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_202: FileType = FileType {
    file_format: &FileFormat {
        id: 202,
        source_type: SourceType::Linguist,
        name: "Linker Script",
        extensions: &["ld", "lds", "x"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
