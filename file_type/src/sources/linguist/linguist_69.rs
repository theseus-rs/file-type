use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_69: FileType = FileType {
    file_format: &FileFormat {
        id: 69,
        source_type: SourceType::Linguist,
        name: "Coq",
        extensions: &["coq", "v"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
