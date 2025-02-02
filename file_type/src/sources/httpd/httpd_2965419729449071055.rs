use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2965419729449071055: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "metalink4 xml",
    extensions: &["meta4"],
    media_types: &["application/metalink4+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
