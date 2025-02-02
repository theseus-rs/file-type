use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12678262227344080642: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "flographit",
    extensions: &["gph"],
    media_types: &["application/vnd.flographit"],
    internal_signatures: &[],
    related_formats: &[],
};
