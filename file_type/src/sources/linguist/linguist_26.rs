use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_26: FileFormat = FileFormat {
    id: 26,
    source_type: SourceType::Linguist,
    name: "AutoHotkey",
    extensions: &["ahk", "ahkl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
