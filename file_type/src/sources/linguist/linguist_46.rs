use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_46: FileType = FileType {
    file_format: &FileFormat {
        id: 46,
        source_type: SourceType::Linguist,
        name: "CLIPS",
        extensions: &["clp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
