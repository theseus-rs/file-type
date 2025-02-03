use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2372984300: FileFormat = FileFormat {
    id: 2_372_984_300,
    source_type: SourceType::Iana,
    name: "json-patch+json",
    extensions: &[],
    media_types: &["application/json-patch+json"],
    internal_signatures: &[],
    related_formats: &[],
};
