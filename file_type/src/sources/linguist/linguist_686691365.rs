use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_686691365: FileType = FileType {
    file_format: &FileFormat {
        id: 686_691_365,
        source_type: SourceType::Linguist,
        name: "Redscript",
        extensions: &["reds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
