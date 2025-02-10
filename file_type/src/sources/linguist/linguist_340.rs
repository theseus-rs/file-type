use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_340: FileType = FileType {
    file_format: &FileFormat {
        id: 340,
        source_type: SourceType::Linguist,
        name: "Sass",
        extensions: &["sass"],
        media_types: &["text/x-sass"],
        signatures: &[],
        related_formats: &[],
    },
};
