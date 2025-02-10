use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_371: FileType = FileType {
    file_format: &FileFormat {
        id: 371,
        source_type: SourceType::Linguist,
        name: "Terra",
        extensions: &["t"],
        media_types: &["text/x-lua"],
        signatures: &[],
        related_formats: &[],
    },
};
