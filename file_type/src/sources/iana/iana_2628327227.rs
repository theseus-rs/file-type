use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2628327227: FileFormat = FileFormat {
    id: 2_628_327_227,
    source_type: SourceType::Iana,
    name: "senml+xml",
    extensions: &[],
    media_types: &["application/senml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
