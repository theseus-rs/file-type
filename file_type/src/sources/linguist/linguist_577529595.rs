use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_577529595: FileType = FileType {
    file_format: &FileFormat {
        id: 577_529_595,
        source_type: SourceType::Linguist,
        name: "4D",
        extensions: &["4dm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
