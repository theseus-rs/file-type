use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5789844385051418491: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rsd xml",
    extensions: &["rsd"],
    media_types: &["application/rsd+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
