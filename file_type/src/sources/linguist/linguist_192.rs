use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_192: FileType = FileType {
    file_format: &FileFormat {
        id: 192,
        source_type: SourceType::Linguist,
        name: "LOLCODE",
        extensions: &["lol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
