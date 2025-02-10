use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_348895984: FileType = FileType {
    file_format: &FileFormat {
        id: 348_895_984,
        source_type: SourceType::Linguist,
        name: "P4",
        extensions: &["p4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
