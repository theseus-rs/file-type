use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_69: FileFormat = FileFormat {
    id: 69,
    source_type: SourceType::Linguist,
    name: "Coq",
    extensions: &["coq", "v"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
