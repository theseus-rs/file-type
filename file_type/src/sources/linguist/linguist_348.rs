use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_348: FileType = FileType {
    file_format: &FileFormat {
        id: 348,
        source_type: SourceType::Linguist,
        name: "Shen",
        extensions: &["shen"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
