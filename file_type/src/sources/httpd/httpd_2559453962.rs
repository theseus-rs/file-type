use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2559453962: FileType = FileType {
    file_format: &FileFormat {
        id: 2_559_453_962,
        source_type: SourceType::Httpd,
        name: "calendar",
        extensions: &["ics", "ifb"],
        media_types: &["text/calendar"],
        signatures: &[],
        related_formats: &[],
    },
};
