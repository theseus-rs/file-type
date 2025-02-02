use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13531758372540882817: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "geogebra file",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[],
};
