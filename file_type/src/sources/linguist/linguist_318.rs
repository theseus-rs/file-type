use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_318: FileType = FileType {
    file_format: &FileFormat {
        id: 318,
        source_type: SourceType::Linguist,
        name: "Raw token data",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
