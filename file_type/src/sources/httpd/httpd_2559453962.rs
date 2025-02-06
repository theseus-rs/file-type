use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2559453962: FileFormat = FileFormat {
    id: 2_559_453_962,
    source_type: SourceType::Httpd,
    name: "calendar",
    extensions: &["ics", "ifb"],
    media_types: &["text/calendar"],
    signatures: &[],
    related_formats: &[],
};
