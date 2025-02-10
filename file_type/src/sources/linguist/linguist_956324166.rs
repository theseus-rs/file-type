use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_956324166: FileType = FileType {
    file_format: &FileFormat {
        id: 956_324_166,
        source_type: SourceType::Linguist,
        name: "Git Attributes",
        extensions: &[],
        media_types: &["text/x-sh"],
        signatures: &[],
        related_formats: &[],
    },
};
