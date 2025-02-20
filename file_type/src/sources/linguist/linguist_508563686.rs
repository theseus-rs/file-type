use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_508563686: FileType = FileType {
    file_format: &FileFormat {
        id: 508_563_686,
        source_type: SourceType::Linguist,
        name: "Vim Help File",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
