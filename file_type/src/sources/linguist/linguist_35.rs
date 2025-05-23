use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_35: FileType = FileType {
    file_format: &FileFormat {
        id: 35,
        source_type: SourceType::Linguist,
        name: "BlitzMax",
        extensions: &["bmx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
