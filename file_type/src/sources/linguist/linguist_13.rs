use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_13: FileType = FileType {
    file_format: &FileFormat {
        id: 13,
        source_type: SourceType::Linguist,
        name: "Alloy",
        extensions: &["als"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
