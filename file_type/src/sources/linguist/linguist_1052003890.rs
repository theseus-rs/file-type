use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1052003890: FileType = FileType {
    file_format: &FileFormat {
        id: 1_052_003_890,
        source_type: SourceType::Linguist,
        name: "KCL",
        extensions: &["k"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
