use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1593479457: FileFormat = FileFormat {
    id: 1_593_479_457,
    source_type: SourceType::Iana,
    name: "pidf-diff+xml",
    extensions: &[],
    media_types: &["application/pidf-diff+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
