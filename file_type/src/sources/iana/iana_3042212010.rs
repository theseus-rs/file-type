use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3042212010: FileFormat = FileFormat {
    id: 3_042_212_010,
    source_type: SourceType::Iana,
    name: "ibe-pkg-reply+xml",
    extensions: &[],
    media_types: &["application/ibe-pkg-reply+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
