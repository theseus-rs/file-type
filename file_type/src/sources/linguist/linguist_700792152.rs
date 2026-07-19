use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_700792152: FileType = FileType {
    file_format: &FileFormat {
        id: 700_792_152,
        source_type: SourceType::Linguist,
        name: "B (Formal Method)",
        extensions: &["mch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
