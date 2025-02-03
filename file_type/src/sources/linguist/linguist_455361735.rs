use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_455361735: FileFormat = FileFormat {
    id: 455_361_735,
    source_type: SourceType::Linguist,
    name: "STL",
    extensions: &["stl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
