use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_440182480: FileType = FileType {
    file_format: &FileFormat {
        id: 440_182_480,
        source_type: SourceType::Linguist,
        name: "Roc",
        extensions: &["roc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
