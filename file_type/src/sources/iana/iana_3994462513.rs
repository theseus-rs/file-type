use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3994462513: FileFormat = FileFormat {
    id: 3_994_462_513,
    source_type: SourceType::Iana,
    name: "font-woff - DEPRECATED in favor of font/woff",
    extensions: &[],
    media_types: &["application/font-woff"],
    internal_signatures: &[],
    related_formats: &[],
};
