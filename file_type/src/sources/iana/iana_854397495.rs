use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_854397495: FileFormat = FileFormat {
    id: 854_397_495,
    source_type: SourceType::Iana,
    name: "parameters",
    extensions: &[],
    media_types: &["text/parameters"],
    internal_signatures: &[],
    related_formats: &[],
};
