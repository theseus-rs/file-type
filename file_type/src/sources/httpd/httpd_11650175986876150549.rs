use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11650175986876150549: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mpegurl",
    extensions: &["m3u"],
    media_types: &["audio/x-mpegurl"],
    internal_signatures: &[],
    related_formats: &[],
};
