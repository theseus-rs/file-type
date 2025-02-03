use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3972709196: FileFormat = FileFormat {
    id: 3_972_709_196,
    source_type: SourceType::Iana,
    name: "vnd.adobe.xdp+xml",
    extensions: &[],
    media_types: &["application/vnd.adobe.xdp+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
