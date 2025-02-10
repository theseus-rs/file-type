use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_20: FileType = FileType {
    file_format: &FileFormat {
        id: 20,
        source_type: SourceType::Linguist,
        name: "Arc",
        extensions: &["arc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
