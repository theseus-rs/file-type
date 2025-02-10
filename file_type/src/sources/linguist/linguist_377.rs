use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_377: FileType = FileType {
    file_format: &FileFormat {
        id: 377,
        source_type: SourceType::Linguist,
        name: "Twig",
        extensions: &["twig"],
        media_types: &["text/x-twig"],
        signatures: &[],
        related_formats: &[],
    },
};
