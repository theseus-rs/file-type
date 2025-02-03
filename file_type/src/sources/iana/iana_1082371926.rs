use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1082371926: FileFormat = FileFormat {
    id: 1_082_371_926,
    source_type: SourceType::Iana,
    name: "vnd.gov.sk.e-form+xml (OBSOLETED by request)",
    extensions: &[],
    media_types: &["application/vnd.gov.sk.e-form+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
