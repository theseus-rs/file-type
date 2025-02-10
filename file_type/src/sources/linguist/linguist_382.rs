use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_382: FileType = FileType {
    file_format: &FileFormat {
        id: 382,
        source_type: SourceType::Linguist,
        name: "UnrealScript",
        extensions: &["uc"],
        media_types: &["text/x-java"],
        signatures: &[],
        related_formats: &[],
    },
};
