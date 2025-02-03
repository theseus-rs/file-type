use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_377: FileFormat = FileFormat {
    id: 377,
    source_type: SourceType::Linguist,
    name: "Twig",
    extensions: &["twig"],
    media_types: &["text/x-twig"],
    internal_signatures: &[],
    related_formats: &[],
};
