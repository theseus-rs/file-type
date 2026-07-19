use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_37496382: FileType = FileType {
    file_format: &FileFormat {
        id: 37_496_382,
        source_type: SourceType::Linguist,
        name: "Power Query",
        extensions: &["pq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
