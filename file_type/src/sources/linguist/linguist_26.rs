use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
