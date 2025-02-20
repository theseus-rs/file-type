use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_424510560: FileType = FileType {
    file_format: &FileFormat {
        id: 424_510_560,
        source_type: SourceType::Linguist,
        name: "STAR",
        extensions: &["star"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
