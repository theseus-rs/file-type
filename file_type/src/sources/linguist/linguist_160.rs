use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_160: FileType = FileType {
    file_format: &FileFormat {
        id: 160,
        source_type: SourceType::Linguist,
        name: "HyPhy",
        extensions: &["bf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
