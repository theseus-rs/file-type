use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_91: FileType = FileType {
    file_format: &FileFormat {
        id: 91,
        source_type: SourceType::Linguist,
        name: "Dylan",
        extensions: &["dyl", "dylan", "intr", "lid"],
        media_types: &["text/x-dylan"],
        signatures: &[],
        related_formats: &[],
    },
};
