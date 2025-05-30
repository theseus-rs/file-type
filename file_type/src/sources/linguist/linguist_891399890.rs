use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_891399890: FileType = FileType {
    file_format: &FileFormat {
        id: 891_399_890,
        source_type: SourceType::Linguist,
        name: "Cairo Zero",
        extensions: &["cairo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
