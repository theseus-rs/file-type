use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_221: FileType = FileType {
    file_format: &FileFormat {
        id: 221,
        source_type: SourceType::Linguist,
        name: "Mako",
        extensions: &["mako", "mao"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
