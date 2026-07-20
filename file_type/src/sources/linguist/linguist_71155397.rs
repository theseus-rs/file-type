use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_71155397: FileType = FileType {
    file_format: &FileFormat {
        id: 71_155_397,
        source_type: SourceType::Linguist,
        name: "CQL",
        extensions: &["cql"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
