use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_441858312: FileType = FileType {
    file_format: &FileFormat {
        id: 441_858_312,
        source_type: SourceType::Linguist,
        name: "Promela",
        extensions: &["pml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
