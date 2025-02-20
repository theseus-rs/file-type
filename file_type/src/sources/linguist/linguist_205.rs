use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_205: FileType = FileType {
    file_format: &FileFormat {
        id: 205,
        source_type: SourceType::Linguist,
        name: "Literate Agda",
        extensions: &["lagda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
