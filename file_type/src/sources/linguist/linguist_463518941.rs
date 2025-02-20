use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_463518941: FileType = FileType {
    file_format: &FileFormat {
        id: 463_518_941,
        source_type: SourceType::Linguist,
        name: "Hare",
        extensions: &["ha"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
