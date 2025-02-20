use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_37531557: FileType = FileType {
    file_format: &FileFormat {
        id: 37_531_557,
        source_type: SourceType::Linguist,
        name: "D2",
        extensions: &["d2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
