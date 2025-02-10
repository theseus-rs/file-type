use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_116: FileType = FileType {
    file_format: &FileFormat {
        id: 116,
        source_type: SourceType::Linguist,
        name: "Frege",
        extensions: &["fr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
