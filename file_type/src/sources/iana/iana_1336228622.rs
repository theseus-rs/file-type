use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1336228622: FileFormat = FileFormat {
    id: 1_336_228_622,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcdata-service-config+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcdata-service-config+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
