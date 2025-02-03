use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_970675279: FileFormat = FileFormat {
    id: 970_675_279,
    source_type: SourceType::Linguist,
    name: "kvlang",
    extensions: &["kv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
