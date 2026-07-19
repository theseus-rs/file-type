use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_592065806: FileType = FileType {
    file_format: &FileFormat {
        id: 592_065_806,
        source_type: SourceType::Linguist,
        name: "IL Assembly",
        extensions: &["il"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
