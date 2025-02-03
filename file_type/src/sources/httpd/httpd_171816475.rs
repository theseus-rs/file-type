use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_171816475: FileFormat = FileFormat {
    id: 171_816_475,
    source_type: SourceType::Httpd,
    name: "lucent voice",
    extensions: &["lvp"],
    media_types: &["audio/vnd.lucent.voice"],
    internal_signatures: &[],
    related_formats: &[],
};
