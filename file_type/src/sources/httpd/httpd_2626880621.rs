use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2626880621: FileFormat = FileFormat {
    id: 2_626_880_621,
    source_type: SourceType::Httpd,
    name: "fluxtime clip",
    extensions: &["ftc"],
    media_types: &["application/vnd.fluxtime.clip"],
    signatures: &[],
    related_formats: &[],
};
