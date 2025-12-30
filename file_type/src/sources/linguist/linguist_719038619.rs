use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_719038619: FileType = FileType {
    file_format: &FileFormat {
        id: 719_038_619,
        source_type: SourceType::Linguist,
        name: "Teal",
        extensions: &["tl"],
        media_types: &["text/x-lua"],
        signatures: &[],
        related_formats: &[],
    },
};
