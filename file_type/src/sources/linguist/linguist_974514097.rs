use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_974514097: FileType = FileType {
    file_format: &FileFormat {
        id: 974_514_097,
        source_type: SourceType::Linguist,
        name: "DataWeave",
        extensions: &["dwl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
