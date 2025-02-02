use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15134871687334348672: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "spotfire dxp",
    extensions: &["dxp"],
    media_types: &["application/vnd.spotfire.dxp"],
    internal_signatures: &[],
    related_formats: &[],
};
