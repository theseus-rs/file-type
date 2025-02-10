use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_44: FileType = FileType {
    file_format: &FileFormat {
        id: 44,
        source_type: SourceType::Linguist,
        name: "C-ObjDump",
        extensions: &["c-objdump"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
