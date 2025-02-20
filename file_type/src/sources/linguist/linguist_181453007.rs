use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_181453007: FileType = FileType {
    file_format: &FileFormat {
        id: 181_453_007,
        source_type: SourceType::Linguist,
        name: "MoonBit",
        extensions: &["mbt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
