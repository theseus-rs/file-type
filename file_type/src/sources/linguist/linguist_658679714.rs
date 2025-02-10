use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_658679714: FileType = FileType {
    file_format: &FileFormat {
        id: 658_679_714,
        source_type: SourceType::Linguist,
        name: "WebVTT",
        extensions: &["vtt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
