use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1055641948: FileType = FileType {
    file_format: &FileFormat {
        id: 1_055_641_948,
        source_type: SourceType::Linguist,
        name: "Vyper",
        extensions: &["vy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
