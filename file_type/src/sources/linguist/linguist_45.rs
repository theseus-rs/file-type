use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_45: FileType = FileType {
    file_format: &FileFormat {
        id: 45,
        source_type: SourceType::Linguist,
        name: "C2hs Haskell",
        extensions: &["chs"],
        media_types: &["text/x-haskell"],
        signatures: &[],
        related_formats: &[],
    },
};
