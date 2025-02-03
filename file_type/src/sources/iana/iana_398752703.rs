use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_398752703: FileFormat = FileFormat {
    id: 398_752_703,
    source_type: SourceType::Iana,
    name: "vnd.sycle+xml",
    extensions: &[],
    media_types: &["application/vnd.sycle+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
