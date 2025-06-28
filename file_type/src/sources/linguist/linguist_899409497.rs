use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_899409497: FileType = FileType {
    file_format: &FileFormat {
        id: 899_409_497,
        source_type: SourceType::Linguist,
        name: "Aiken",
        extensions: &["ak"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
