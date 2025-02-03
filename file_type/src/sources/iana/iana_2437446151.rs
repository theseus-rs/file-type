use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2437446151: FileFormat = FileFormat {
    id: 2_437_446_151,
    source_type: SourceType::Iana,
    name: "geoxacml+json",
    extensions: &[],
    media_types: &["application/geoxacml+json"],
    internal_signatures: &[],
    related_formats: &[],
};
