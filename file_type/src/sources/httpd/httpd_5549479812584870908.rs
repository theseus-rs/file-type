use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5549479812584870908: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mediaservercontrol xml",
    extensions: &["mscml"],
    media_types: &["application/mediaservercontrol+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
