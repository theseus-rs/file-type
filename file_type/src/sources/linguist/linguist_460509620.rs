use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_460509620: FileType = FileType {
    file_format: &FileFormat {
        id: 460_509_620,
        source_type: SourceType::Linguist,
        name: "Edge",
        extensions: &["edge"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
