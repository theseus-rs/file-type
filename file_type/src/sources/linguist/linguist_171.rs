use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_171: FileType = FileType {
    file_format: &FileFormat {
        id: 171,
        source_type: SourceType::Linguist,
        name: "Isabelle ROOT",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
