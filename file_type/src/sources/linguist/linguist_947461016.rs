use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_947461016: FileType = FileType {
    file_format: &FileFormat {
        id: 947_461_016,
        source_type: SourceType::Linguist,
        name: "Go Module",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
