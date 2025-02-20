use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_140: FileType = FileType {
    file_format: &FileFormat {
        id: 140,
        source_type: SourceType::Linguist,
        name: "Graphviz (DOT)",
        extensions: &["dot", "gv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
