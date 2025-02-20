use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_252961827: FileType = FileType {
    file_format: &FileFormat {
        id: 252_961_827,
        source_type: SourceType::Linguist,
        name: "Pyret",
        extensions: &["arr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
