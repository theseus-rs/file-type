use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1936666943: FileFormat = FileFormat {
    id: 1_936_666_943,
    source_type: SourceType::Httpd,
    name: "kodak descriptor",
    extensions: &["sse"],
    media_types: &["application/vnd.kodak-descriptor"],
    internal_signatures: &[],
    related_formats: &[],
};
