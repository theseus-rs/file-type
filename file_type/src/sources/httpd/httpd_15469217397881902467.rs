use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15469217397881902467: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "install instructions",
    extensions: &["install"],
    media_types: &["application/x-install-instructions"],
    internal_signatures: &[],
    related_formats: &[],
};
