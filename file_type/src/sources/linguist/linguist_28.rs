use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_28: FileType = FileType {
    file_format: &FileFormat {
        id: 28,
        source_type: SourceType::Linguist,
        name: "Awk",
        extensions: &["auk", "awk", "gawk", "mawk", "nawk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
