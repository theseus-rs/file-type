use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_437786028: FileFormat = FileFormat {
    id: 437_786_028,
    source_type: SourceType::Iana,
    name: "vnd.cmles.radio-events",
    extensions: &[],
    media_types: &["audio/vnd.cmles.radio-events"],
    internal_signatures: &[],
    related_formats: &[],
};
