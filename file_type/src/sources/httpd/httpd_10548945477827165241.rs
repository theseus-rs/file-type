use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10548945477827165241: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sus calendar",
    extensions: &["sus", "susp"],
    media_types: &["application/vnd.sus-calendar"],
    internal_signatures: &[],
    related_formats: &[],
};
