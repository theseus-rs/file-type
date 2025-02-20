use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_481192983: FileType = FileType {
    file_format: &FileFormat {
        id: 481_192_983,
        source_type: SourceType::Linguist,
        name: "NEON",
        extensions: &["neon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
