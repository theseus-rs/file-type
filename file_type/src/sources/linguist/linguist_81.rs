use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_81: FileType = FileType {
    file_format: &FileFormat {
        id: 81,
        source_type: SourceType::Linguist,
        name: "D-ObjDump",
        extensions: &["d-objdump"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
