use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_211: FileType = FileType {
    file_format: &FileFormat {
        id: 211,
        source_type: SourceType::Linguist,
        name: "LookML",
        extensions: &["lkml", "lookml"],
        media_types: &["text/x-yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
