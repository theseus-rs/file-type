use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_301: FileType = FileType {
    file_format: &FileFormat {
        id: 301,
        source_type: SourceType::Linguist,
        name: "PureBasic",
        extensions: &["pb", "pbi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
