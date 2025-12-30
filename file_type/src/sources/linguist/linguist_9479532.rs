use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_9479532: FileType = FileType {
    file_format: &FileFormat {
        id: 9_479_532,
        source_type: SourceType::Linguist,
        name: "KFramework",
        extensions: &["k"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
