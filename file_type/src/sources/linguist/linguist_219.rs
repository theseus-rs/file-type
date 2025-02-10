use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_219: FileType = FileType {
    file_format: &FileFormat {
        id: 219,
        source_type: SourceType::Linguist,
        name: "MUF",
        extensions: &["m", "muf"],
        media_types: &["text/x-forth"],
        signatures: &[],
        related_formats: &[],
    },
};
