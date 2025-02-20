use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_408: FileType = FileType {
    file_format: &FileFormat {
        id: 408,
        source_type: SourceType::Linguist,
        name: "YANG",
        extensions: &["yang"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
