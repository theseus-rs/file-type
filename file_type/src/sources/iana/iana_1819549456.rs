use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1819549456: FileFormat = FileFormat {
    id: 1_819_549_456,
    source_type: SourceType::Iana,
    name: "SMV-QCP",
    extensions: &[],
    media_types: &["audio/SMV-QCP"],
    internal_signatures: &[],
    related_formats: &[],
};
