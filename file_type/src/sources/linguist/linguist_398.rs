use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_398: FileType = FileType {
    file_format: &FileFormat {
        id: 398,
        source_type: SourceType::Linguist,
        name: "XC",
        extensions: &["xc"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};
