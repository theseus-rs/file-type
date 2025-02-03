use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_293173837: FileFormat = FileFormat {
    id: 293_173_837,
    source_type: SourceType::Iana,
    name: "problem+json",
    extensions: &[],
    media_types: &["application/problem+json"],
    internal_signatures: &[],
    related_formats: &[],
};
