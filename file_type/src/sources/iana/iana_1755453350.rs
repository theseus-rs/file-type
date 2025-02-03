use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1755453350: FileFormat = FileFormat {
    id: 1_755_453_350,
    source_type: SourceType::Iana,
    name: "L24",
    extensions: &[],
    media_types: &["audio/L24"],
    internal_signatures: &[],
    related_formats: &[],
};
