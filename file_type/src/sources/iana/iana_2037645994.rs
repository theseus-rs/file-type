use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2037645994: FileFormat = FileFormat {
    id: 2_037_645_994,
    source_type: SourceType::Iana,
    name: "vnd.ga4gh.passport+jwt",
    extensions: &[],
    media_types: &["application/vnd.ga4gh.passport+jwt"],
    signatures: &[],
    related_formats: &[],
};
