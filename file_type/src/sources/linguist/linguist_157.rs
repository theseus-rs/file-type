use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_157: FileType = FileType {
    file_format: &FileFormat {
        id: 157,
        source_type: SourceType::Linguist,
        name: "Haskell",
        extensions: &["hs", "hs-boot", "hsc"],
        media_types: &["text/x-haskell"],
        signatures: &[],
        related_formats: &[],
    },
};
