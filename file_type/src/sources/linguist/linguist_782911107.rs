use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_782911107: FileType = FileType {
    file_format: &FileFormat {
        id: 782_911_107,
        source_type: SourceType::Linguist,
        name: "X BitMap",
        extensions: &["xbm"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};
