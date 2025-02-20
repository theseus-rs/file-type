use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_305313959: FileType = FileType {
    file_format: &FileFormat {
        id: 305_313_959,
        source_type: SourceType::Linguist,
        name: "Procfile",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
