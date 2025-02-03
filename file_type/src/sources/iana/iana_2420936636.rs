use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2420936636: FileFormat = FileFormat {
    id: 2_420_936_636,
    source_type: SourceType::Iana,
    name: "provided-claims+jwt",
    extensions: &[],
    media_types: &["application/provided-claims+jwt"],
    internal_signatures: &[],
    related_formats: &[],
};
