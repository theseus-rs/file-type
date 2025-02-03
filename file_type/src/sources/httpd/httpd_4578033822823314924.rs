use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4578033822823314924: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fluxtime clip",
    extensions: &["ftc"],
    media_types: &["application/vnd.fluxtime.clip"],
    internal_signatures: &[],
    related_formats: &[],
};
