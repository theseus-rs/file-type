use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_205455127: FileFormat = FileFormat {
    id: 205_455_127,
    source_type: SourceType::Iana,
    name: "EVRC-QCP",
    extensions: &[],
    media_types: &["audio/EVRC-QCP"],
    internal_signatures: &[],
    related_formats: &[],
};
