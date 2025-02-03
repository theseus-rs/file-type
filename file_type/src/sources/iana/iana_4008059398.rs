use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4008059398: FileFormat = FileFormat {
    id: 4_008_059_398,
    source_type: SourceType::Iana,
    name: "mud+json",
    extensions: &[],
    media_types: &["application/mud+json"],
    internal_signatures: &[],
    related_formats: &[],
};
