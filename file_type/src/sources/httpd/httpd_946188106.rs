use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_946188106: FileFormat = FileFormat {
    id: 946_188_106,
    source_type: SourceType::Httpd,
    name: "vcalendar",
    extensions: &["vcs"],
    media_types: &["text/x-vcalendar"],
    internal_signatures: &[],
    related_formats: &[],
};
