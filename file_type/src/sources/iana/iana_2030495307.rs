use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2030495307: FileFormat = FileFormat {
    id: 2_030_495_307,
    source_type: SourceType::Iana,
    name: "vnd.apple.installer+xml",
    extensions: &[],
    media_types: &["application/vnd.apple.installer+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
