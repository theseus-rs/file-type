use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_264: FileFormat = FileFormat {
    id: 264,
    source_type: SourceType::Linguist,
    name: "OpenEdge ABL",
    extensions: &["cls", "p", "w"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
