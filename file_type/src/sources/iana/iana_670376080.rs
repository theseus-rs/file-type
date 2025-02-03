use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_670376080: FileFormat = FileFormat {
    id: 670_376_080,
    source_type: SourceType::Iana,
    name: "sp-midi",
    extensions: &[],
    media_types: &["audio/sp-midi"],
    internal_signatures: &[],
    related_formats: &[],
};
