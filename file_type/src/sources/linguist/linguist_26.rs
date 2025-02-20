use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_26: FileType = FileType {
    file_format: &FileFormat {
        id: 26,
        source_type: SourceType::Linguist,
        name: "AutoHotkey",
        extensions: &["ahk", "ahkl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
