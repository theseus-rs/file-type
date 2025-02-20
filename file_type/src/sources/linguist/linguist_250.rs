use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_250: FileType = FileType {
    file_format: &FileFormat {
        id: 250,
        source_type: SourceType::Linguist,
        name: "Ninja",
        extensions: &["ninja"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
