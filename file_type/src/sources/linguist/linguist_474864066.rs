use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_474864066: FileType = FileType {
    file_format: &FileFormat {
        id: 474_864_066,
        source_type: SourceType::Linguist,
        name: "Muse",
        extensions: &["muse"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
