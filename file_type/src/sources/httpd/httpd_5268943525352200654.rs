use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5268943525352200654: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "h264",
    extensions: &["h264"],
    media_types: &["video/h264"],
    internal_signatures: &[],
    related_formats: &[],
};
