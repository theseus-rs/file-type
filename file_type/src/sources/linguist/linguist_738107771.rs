use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_738107771: FileType = FileType {
    file_format: &FileFormat {
        id: 738_107_771,
        source_type: SourceType::Linguist,
        name: "Godot Resource",
        extensions: &["gdnlib", "gdns", "tres", "tscn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
