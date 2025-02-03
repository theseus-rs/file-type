use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3158562477: FileFormat = FileFormat {
    id: 3_158_562_477,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.current-location-discovery+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.current-location-discovery+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
