use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9915205638149722777: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "xara",
    extensions: &["xar"],
    media_types: &["application/vnd.xara"],
    internal_signatures: &[],
    related_formats: &[],
};
