use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1948177638: FileFormat = FileFormat {
    id: 1_948_177_638,
    source_type: SourceType::Iana,
    name: "tone",
    extensions: &[],
    media_types: &["audio/tone"],
    internal_signatures: &[],
    related_formats: &[],
};
