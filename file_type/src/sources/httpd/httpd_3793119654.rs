use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3793119654: FileFormat = FileFormat {
    id: 3_793_119_654,
    source_type: SourceType::Httpd,
    name: "ms officetheme",
    extensions: &["thmx"],
    media_types: &["application/vnd.ms-officetheme"],
    internal_signatures: &[],
    related_formats: &[],
};
