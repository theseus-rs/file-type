use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_133: FileType = FileType {
    file_format: &FileFormat {
        id: 133,
        source_type: SourceType::Linguist,
        name: "Golo",
        extensions: &["golo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
