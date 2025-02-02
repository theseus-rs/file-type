use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10710046949140717356: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mozilla xul xml",
    extensions: &["xul"],
    media_types: &["application/vnd.mozilla.xul+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
