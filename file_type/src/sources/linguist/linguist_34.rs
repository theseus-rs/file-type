use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_34: FileType = FileType {
    file_format: &FileFormat {
        id: 34,
        source_type: SourceType::Linguist,
        name: "BlitzBasic",
        extensions: &["bb", "decls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
