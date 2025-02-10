use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_106029007: FileType = FileType {
    file_format: &FileFormat {
        id: 106_029_007,
        source_type: SourceType::Linguist,
        name: "Praat",
        extensions: &["praat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
