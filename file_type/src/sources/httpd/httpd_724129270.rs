use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_724129270: FileFormat = FileFormat {
    id: 724_129_270,
    source_type: SourceType::Httpd,
    name: "h264",
    extensions: &["h264"],
    media_types: &["video/h264"],
    internal_signatures: &[],
    related_formats: &[],
};
