use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2480952760: FileFormat = FileFormat {
    id: 2_480_952_760,
    source_type: SourceType::Iana,
    name: "eat-bun+json",
    extensions: &[],
    media_types: &["application/eat-bun+json"],
    internal_signatures: &[],
    related_formats: &[],
};
