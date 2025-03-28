use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_120: FileType = FileType {
    file_format: &FileFormat {
        id: 120,
        source_type: SourceType::Linguist,
        name: "Unix Assembly",
        extensions: &["ms", "s"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
