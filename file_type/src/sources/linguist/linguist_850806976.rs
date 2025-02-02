use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_850806976: FileFormat = FileFormat {
    id: 850_806_976,
    source_type: SourceType::Linguist,
    name: "Cypher",
    extensions: &["cyp", "cypher"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
