use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_196: FileType = FileType {
    file_format: &FileFormat {
        id: 196,
        source_type: SourceType::Linguist,
        name: "Latte",
        extensions: &["latte"],
        media_types: &["text/x-smarty"],
        signatures: &[],
        related_formats: &[],
    },
};
