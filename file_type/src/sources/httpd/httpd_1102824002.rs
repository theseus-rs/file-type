use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1102824002: FileFormat = FileFormat {
    id: 1_102_824_002,
    source_type: SourceType::Httpd,
    name: "vivo",
    extensions: &["viv"],
    media_types: &["video/vnd.vivo"],
    internal_signatures: &[],
    related_formats: &[],
};
