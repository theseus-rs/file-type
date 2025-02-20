use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_603336474: FileType = FileType {
    file_format: &FileFormat {
        id: 603_336_474,
        source_type: SourceType::Linguist,
        name: "KakouneScript",
        extensions: &["kak"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
