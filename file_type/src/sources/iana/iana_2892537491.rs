use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2892537491: FileFormat = FileFormat {
    id: 2_892_537_491,
    source_type: SourceType::Iana,
    name: "vnd.smaf",
    extensions: &[],
    media_types: &["application/vnd.smaf"],
    internal_signatures: &[],
    related_formats: &[],
};
