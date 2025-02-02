use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_925235833: FileFormat = FileFormat {
    id: 925_235_833,
    source_type: SourceType::Linguist,
    name: "EdgeQL",
    extensions: &["edgeql", "esdl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
