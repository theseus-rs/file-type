use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_280: FileType = FileType {
    file_format: &FileFormat {
        id: 280,
        source_type: SourceType::Linguist,
        name: "Parrot Internal Representation",
        extensions: &["pir"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
