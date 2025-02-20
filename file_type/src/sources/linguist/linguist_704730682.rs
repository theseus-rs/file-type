use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_704730682: FileType = FileType {
    file_format: &FileFormat {
        id: 704_730_682,
        source_type: SourceType::Linguist,
        name: "Typst",
        extensions: &["typ"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
