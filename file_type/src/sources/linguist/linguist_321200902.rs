use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_321200902: FileType = FileType {
    file_format: &FileFormat {
        id: 321_200_902,
        source_type: SourceType::Linguist,
        name: "Bicep",
        extensions: &["bicep", "bicepparam"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
