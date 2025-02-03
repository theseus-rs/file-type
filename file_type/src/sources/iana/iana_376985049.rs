use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_376985049: FileFormat = FileFormat {
    id: 376_985_049,
    source_type: SourceType::Iana,
    name: "scaip+xml",
    extensions: &[],
    media_types: &["application/scaip+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
