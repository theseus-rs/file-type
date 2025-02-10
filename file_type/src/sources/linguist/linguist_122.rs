use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_122: FileType = FileType {
    file_format: &FileFormat {
        id: 122,
        source_type: SourceType::Linguist,
        name: "GDB",
        extensions: &["gdb", "gdbinit"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
