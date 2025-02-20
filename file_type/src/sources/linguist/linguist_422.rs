use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_422: FileType = FileType {
    file_format: &FileFormat {
        id: 422,
        source_type: SourceType::Linguist,
        name: "TI Program",
        extensions: &["8xp", "8xp.txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
