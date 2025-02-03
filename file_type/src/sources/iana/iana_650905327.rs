use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_650905327: FileFormat = FileFormat {
    id: 650_905_327,
    source_type: SourceType::Iana,
    name: "xcon-conference-info-diff+xml",
    extensions: &[],
    media_types: &["application/xcon-conference-info-diff+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
