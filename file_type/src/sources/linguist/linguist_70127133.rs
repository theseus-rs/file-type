use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_70127133: FileType = FileType {
    file_format: &FileFormat {
        id: 70_127_133,
        source_type: SourceType::Linguist,
        name: "Jai",
        extensions: &["jai"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
