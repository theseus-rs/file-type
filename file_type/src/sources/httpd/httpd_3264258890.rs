use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3264258890: FileFormat = FileFormat {
    id: 3_264_258_890,
    source_type: SourceType::Httpd,
    name: "yamaha smaf audio",
    extensions: &["saf"],
    media_types: &["application/vnd.yamaha.smaf-audio"],
    internal_signatures: &[],
    related_formats: &[],
};
