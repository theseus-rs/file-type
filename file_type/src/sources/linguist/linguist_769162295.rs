use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_769162295: FileType = FileType {
    file_format: &FileFormat {
        id: 769_162_295,
        source_type: SourceType::Linguist,
        name: "TMDL",
        extensions: &["tmdl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
