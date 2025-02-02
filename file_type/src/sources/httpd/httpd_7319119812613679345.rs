use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7319119812613679345: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "voicexml xml",
    extensions: &["vxml"],
    media_types: &["application/voicexml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
