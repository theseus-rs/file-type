use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_110: FileType = FileType {
    file_format: &FileFormat {
        id: 110,
        source_type: SourceType::Linguist,
        name: "Fantom",
        extensions: &["fan"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
