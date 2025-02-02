use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_140: FileFormat = FileFormat {
    id: 140,
    source_type: SourceType::Linguist,
    name: "Graphviz (DOT)",
    extensions: &["dot", "gv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
