use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2953193598: FileFormat = FileFormat {
    id: 2_953_193_598,
    source_type: SourceType::Iana,
    name: "yang-data+json",
    extensions: &[],
    media_types: &["application/yang-data+json"],
    internal_signatures: &[],
    related_formats: &[],
};
