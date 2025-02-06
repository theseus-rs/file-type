use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1300491: FileFormat = FileFormat {
    id: 1_300_491,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcdata-payload",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcdata-payload"],
    signatures: &[],
    related_formats: &[],
};
