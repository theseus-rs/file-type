use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_128447695: FileType = FileType {
    file_format: &FileFormat {
        id: 128_447_695,
        source_type: SourceType::Linguist,
        name: "Just",
        extensions: &["just"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
