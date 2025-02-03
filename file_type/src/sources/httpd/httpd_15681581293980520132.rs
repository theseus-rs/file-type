use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15681581293980520132: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mpegurl",
    extensions: &["mxu", "m4u"],
    media_types: &["video/vnd.mpegurl"],
    internal_signatures: &[],
    related_formats: &[],
};
