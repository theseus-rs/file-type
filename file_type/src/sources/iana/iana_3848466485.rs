use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3848466485: FileFormat = FileFormat {
    id: 3_848_466_485,
    source_type: SourceType::Iana,
    name: "font-sfnt - DEPRECATED in favor of font/sfnt",
    extensions: &[],
    media_types: &["application/font-sfnt"],
    internal_signatures: &[],
    related_formats: &[],
};
