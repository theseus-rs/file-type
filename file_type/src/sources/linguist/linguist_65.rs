use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_65: FileType = FileType {
    file_format: &FileFormat {
        id: 65,
        source_type: SourceType::Linguist,
        name: "ColdFusion CFC",
        extensions: &["cfc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
