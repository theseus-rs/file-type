use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_684385621: FileType = FileType {
    file_format: &FileFormat {
        id: 684_385_621,
        source_type: SourceType::Linguist,
        name: "Pip Requirements",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
