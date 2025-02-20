use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_180: FileType = FileType {
    file_format: &FileFormat {
        id: 180,
        source_type: SourceType::Linguist,
        name: "Jasmin",
        extensions: &["j"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
