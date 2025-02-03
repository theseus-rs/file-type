use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6306876490844253642: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "calendar",
    extensions: &["ics", "ifb"],
    media_types: &["text/calendar"],
    internal_signatures: &[],
    related_formats: &[],
};
