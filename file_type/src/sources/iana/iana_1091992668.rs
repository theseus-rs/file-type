use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1091992668: FileFormat = FileFormat {
    id: 1_091_992_668,
    source_type: SourceType::Iana,
    name: "vnd.nuera.ecelp9600",
    extensions: &[],
    media_types: &["audio/vnd.nuera.ecelp9600"],
    internal_signatures: &[],
    related_formats: &[],
};
