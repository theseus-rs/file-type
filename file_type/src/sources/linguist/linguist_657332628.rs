use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_657332628: FileType = FileType {
    file_format: &FileFormat {
        id: 657_332_628,
        source_type: SourceType::Linguist,
        name: "CWeb",
        extensions: &["w"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
