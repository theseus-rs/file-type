use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2929592017: FileFormat = FileFormat {
    id: 2_929_592_017,
    source_type: SourceType::Iana,
    name: "ttml+xml",
    extensions: &[],
    media_types: &["application/ttml+xml"],
    signatures: &[],
    related_formats: &[],
};
