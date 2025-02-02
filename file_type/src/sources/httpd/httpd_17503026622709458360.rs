use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17503026622709458360: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fpx",
    extensions: &["fpx"],
    media_types: &["image/vnd.fpx"],
    internal_signatures: &[],
    related_formats: &[],
};
