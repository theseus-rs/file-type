use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_89289301: FileFormat = FileFormat {
    id: 89_289_301,
    source_type: SourceType::Linguist,
    name: "TSPLIB data",
    extensions: &["tsp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
