use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_271: FileType = FileType {
    file_format: &FileFormat {
        id: 271,
        source_type: SourceType::Linguist,
        name: "Pawn",
        extensions: &["inc", "pwn", "sma"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
