use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_373: FileType = FileType {
    file_format: &FileFormat {
        id: 373,
        source_type: SourceType::Linguist,
        name: "Textile",
        extensions: &["textile"],
        media_types: &["text/x-textile"],
        signatures: &[],
        related_formats: &[],
    },
};
