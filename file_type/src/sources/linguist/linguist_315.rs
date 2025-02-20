use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_315: FileType = FileType {
    file_format: &FileFormat {
        id: 315,
        source_type: SourceType::Linguist,
        name: "RUNOFF",
        extensions: &["rnh", "rno"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
