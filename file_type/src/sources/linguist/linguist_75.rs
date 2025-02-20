use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_75: FileType = FileType {
    file_format: &FileFormat {
        id: 75,
        source_type: SourceType::Linguist,
        name: "Csound Score",
        extensions: &["sco"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
