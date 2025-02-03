use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13986790209176469882: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "hp pclxl",
    extensions: &["pclxl"],
    media_types: &["application/vnd.hp-pclxl"],
    internal_signatures: &[],
    related_formats: &[],
};
