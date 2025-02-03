use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6202298358182500671: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "curl",
    extensions: &["curl"],
    media_types: &["text/vnd.curl"],
    internal_signatures: &[],
    related_formats: &[],
};
