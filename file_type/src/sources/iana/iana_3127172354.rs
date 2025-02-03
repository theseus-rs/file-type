use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3127172354: FileFormat = FileFormat {
    id: 3_127_172_354,
    source_type: SourceType::Iana,
    name: "c2pa",
    extensions: &[],
    media_types: &["application/c2pa"],
    internal_signatures: &[],
    related_formats: &[],
};
