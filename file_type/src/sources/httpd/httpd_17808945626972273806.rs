use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17808945626972273806: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "oasis opendocument graphics",
    extensions: &["odg"],
    media_types: &["application/vnd.oasis.opendocument.graphics"],
    internal_signatures: &[],
    related_formats: &[],
};
