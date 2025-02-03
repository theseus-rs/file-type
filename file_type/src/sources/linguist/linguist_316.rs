use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_316: FileFormat = FileFormat {
    id: 316,
    source_type: SourceType::Linguist,
    name: "Racket",
    extensions: &["rkt", "rktd", "rktl", "scrbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
