use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_562056483: FileType = FileType {
    file_format: &FileFormat {
        id: 562_056_483,
        source_type: SourceType::Linguist,
        name: "Quint",
        extensions: &["qnt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
