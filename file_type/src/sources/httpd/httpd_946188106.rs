use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_946188106: FileType = FileType {
    file_format: &FileFormat {
        id: 946_188_106,
        source_type: SourceType::Httpd,
        name: "vcalendar",
        extensions: &["vcs"],
        media_types: &["text/x-vcalendar"],
        signatures: &[],
        related_formats: &[],
    },
};
