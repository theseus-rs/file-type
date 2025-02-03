use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1652132669: FileFormat = FileFormat {
    id: 1_652_132_669,
    source_type: SourceType::Iana,
    name: "geoxacml+xml",
    extensions: &[],
    media_types: &["application/geoxacml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
