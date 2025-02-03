use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2650455745: FileFormat = FileFormat {
    id: 2_650_455_745,
    source_type: SourceType::Iana,
    name: "captive+json",
    extensions: &[],
    media_types: &["application/captive+json"],
    internal_signatures: &[],
    related_formats: &[],
};
