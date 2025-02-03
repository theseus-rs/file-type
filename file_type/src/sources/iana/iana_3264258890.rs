use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3264258890: FileFormat = FileFormat {
    id: 3_264_258_890,
    source_type: SourceType::Iana,
    name: "vnd.yamaha.smaf-audio",
    extensions: &[],
    media_types: &["application/vnd.yamaha.smaf-audio"],
    internal_signatures: &[],
    related_formats: &[],
};
