use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_74444240: FileType = FileType {
    file_format: &FileFormat {
        id: 74_444_240,
        source_type: SourceType::Linguist,
        name: "Ignore List",
        extensions: &["gitignore"],
        media_types: &["text/x-sh"],
        signatures: &[],
        related_formats: &[],
    },
};
