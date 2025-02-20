use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_8: FileType = FileType {
    file_format: &FileFormat {
        id: 8,
        source_type: SourceType::Linguist,
        name: "Classic ASP",
        extensions: &["asp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
