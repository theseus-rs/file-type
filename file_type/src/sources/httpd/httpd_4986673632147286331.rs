use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4986673632147286331: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ahead space",
    extensions: &["ahead"],
    media_types: &["application/vnd.ahead.space"],
    internal_signatures: &[],
    related_formats: &[],
};
