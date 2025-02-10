use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_354: FileType = FileType {
    file_format: &FileFormat {
        id: 354,
        source_type: SourceType::Linguist,
        name: "SourcePawn",
        extensions: &["inc", "sp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
