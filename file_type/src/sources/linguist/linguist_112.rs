use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_112: FileType = FileType {
    file_format: &FileFormat {
        id: 112,
        source_type: SourceType::Linguist,
        name: "Filterscript",
        extensions: &["fs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
