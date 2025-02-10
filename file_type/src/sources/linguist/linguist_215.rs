use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_215: FileType = FileType {
    file_format: &FileFormat {
        id: 215,
        source_type: SourceType::Linguist,
        name: "M4",
        extensions: &["m4", "mc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
