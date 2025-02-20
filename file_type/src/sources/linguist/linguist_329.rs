use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_329: FileType = FileType {
    file_format: &FileFormat {
        id: 329,
        source_type: SourceType::Linguist,
        name: "SCSS",
        extensions: &["scss"],
        media_types: &["text/x-scss"],
        signatures: &[],
        related_formats: &[],
    },
};
