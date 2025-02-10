use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_256: FileType = FileType {
    file_format: &FileFormat {
        id: 256,
        source_type: SourceType::Linguist,
        name: "ObjDump",
        extensions: &["objdump"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
