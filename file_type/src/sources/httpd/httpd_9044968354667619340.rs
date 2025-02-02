use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9044968354667619340: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "apple mpegurl",
    extensions: &["m3u8"],
    media_types: &["application/vnd.apple.mpegurl"],
    internal_signatures: &[],
    related_formats: &[],
};
