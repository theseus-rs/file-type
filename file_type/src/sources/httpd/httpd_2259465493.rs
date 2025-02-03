use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2259465493: FileFormat = FileFormat {
    id: 2_259_465_493,
    source_type: SourceType::Httpd,
    name: "tgif",
    extensions: &["obj"],
    media_types: &["application/x-tgif"],
    internal_signatures: &[],
    related_formats: &[],
};
