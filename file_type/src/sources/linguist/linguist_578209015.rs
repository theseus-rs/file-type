use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_578209015: FileFormat = FileFormat {
    id: 578_209_015,
    source_type: SourceType::Linguist,
    name: "Astro",
    extensions: &["astro"],
    media_types: &["text/jsx"],
    internal_signatures: &[],
    related_formats: &[],
};
