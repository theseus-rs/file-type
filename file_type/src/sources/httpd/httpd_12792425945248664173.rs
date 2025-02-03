use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12792425945248664173: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sdp",
    extensions: &["sdp"],
    media_types: &["application/sdp"],
    internal_signatures: &[],
    related_formats: &[],
};
