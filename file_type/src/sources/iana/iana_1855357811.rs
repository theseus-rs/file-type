use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1855357811: FileFormat = FileFormat {
    id: 1_855_357_811,
    source_type: SourceType::Iana,
    name: "nlsml+xml",
    extensions: &[],
    media_types: &["application/nlsml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
