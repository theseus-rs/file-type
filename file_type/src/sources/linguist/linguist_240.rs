use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_240: FileType = FileType {
    file_format: &FileFormat {
        id: 240,
        source_type: SourceType::Linguist,
        name: "NCL",
        extensions: &["ncl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
