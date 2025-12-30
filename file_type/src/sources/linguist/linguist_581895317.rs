use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_581895317: FileType = FileType {
    file_format: &FileFormat {
        id: 581_895_317,
        source_type: SourceType::Linguist,
        name: "Cangjie",
        extensions: &["cj"],
        media_types: &["text/x-swift"],
        signatures: &[],
        related_formats: &[],
    },
};
