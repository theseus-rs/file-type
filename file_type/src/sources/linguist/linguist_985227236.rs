use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_985227236: FileType = FileType {
    file_format: &FileFormat {
        id: 985_227_236,
        source_type: SourceType::Linguist,
        name: "Object Data Instance Notation",
        extensions: &["odin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
