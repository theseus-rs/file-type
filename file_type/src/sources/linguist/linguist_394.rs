use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_394: FileType = FileType {
    file_format: &FileFormat {
        id: 394,
        source_type: SourceType::Linguist,
        name: "Web Ontology Language",
        extensions: &["owl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
