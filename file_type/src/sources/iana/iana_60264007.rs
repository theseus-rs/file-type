use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_60264007: FileFormat = FileFormat {
    id: 60_264_007,
    source_type: SourceType::Iana,
    name: "xcap-att+xml",
    extensions: &[],
    media_types: &["application/xcap-att+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
