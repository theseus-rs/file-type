use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_632765617: FileType = FileType {
    file_format: &FileFormat {
        id: 632_765_617,
        source_type: SourceType::Linguist,
        name: "Type Language",
        extensions: &["tl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
