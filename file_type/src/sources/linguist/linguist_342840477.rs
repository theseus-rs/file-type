use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_342840477: FileFormat = FileFormat {
    id: 342_840_477,
    source_type: SourceType::Linguist,
    name: "Easybuild",
    extensions: &["eb"],
    media_types: &["text/x-python"],
    internal_signatures: &[],
    related_formats: &[],
};
