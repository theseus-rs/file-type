use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_311: FileType = FileType {
    file_format: &FileFormat {
        id: 311,
        source_type: SourceType::Linguist,
        name: "REXX",
        extensions: &["pprx", "rex", "rexx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
