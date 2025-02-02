use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14166596980345473346: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "otf",
    extensions: &["otf"],
    media_types: &["font/otf"],
    internal_signatures: &[],
    related_formats: &[],
};
