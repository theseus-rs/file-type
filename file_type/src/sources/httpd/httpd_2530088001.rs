use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2530088001: FileFormat = FileFormat {
    id: 2_530_088_001,
    source_type: SourceType::Httpd,
    name: "voicexml xml",
    extensions: &["vxml"],
    media_types: &["application/voicexml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
