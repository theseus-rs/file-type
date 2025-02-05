use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2829941986: FileFormat = FileFormat {
    id: 2_829_941_986,
    source_type: SourceType::Iana,
    name: "taxii+json",
    extensions: &[],
    media_types: &["application/taxii+json"],
    signatures: &[],
    related_formats: &[],
};
