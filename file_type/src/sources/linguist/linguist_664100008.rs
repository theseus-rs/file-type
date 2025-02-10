use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_664100008: FileType = FileType {
    file_format: &FileFormat {
        id: 664_100_008,
        source_type: SourceType::Linguist,
        name: "OMNeT++ MSG",
        extensions: &["msg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
