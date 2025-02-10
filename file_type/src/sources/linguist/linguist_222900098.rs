use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_222900098: FileType = FileType {
    file_format: &FileFormat {
        id: 222_900_098,
        source_type: SourceType::Linguist,
        name: "Soong",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
