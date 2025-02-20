use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_264: FileType = FileType {
    file_format: &FileFormat {
        id: 264,
        source_type: SourceType::Linguist,
        name: "OpenEdge ABL",
        extensions: &["cls", "p", "w"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
