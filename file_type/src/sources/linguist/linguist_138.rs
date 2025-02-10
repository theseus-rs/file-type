use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_138: FileType = FileType {
    file_format: &FileFormat {
        id: 138,
        source_type: SourceType::Linguist,
        name: "Graph Modeling Language",
        extensions: &["gml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
