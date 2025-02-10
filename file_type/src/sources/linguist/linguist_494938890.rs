use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_494938890: FileType = FileType {
    file_format: &FileFormat {
        id: 494_938_890,
        source_type: SourceType::Linguist,
        name: "ZenScript",
        extensions: &["zs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
