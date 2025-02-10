use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
